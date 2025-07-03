use holochain_types::prelude::AppBundle;
use std::path::PathBuf;
use tauri_plugin_holochain::{HolochainPluginConfig, HolochainExt, NetworkConfig, vec_to_locked};
use tauri_plugin_opener::OpenerExt;
use tauri::{AppHandle, Manager};
#[cfg(not(mobile))]
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
#[cfg(not(mobile))]
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};

const APP_ID: &'static str = "domino";
const NETWORK_SEED: &'static str = "domino-sandbox";

pub fn happ_bundle() -> AppBundle {
    let bytes = include_bytes!("../../domino/workdir/domino.happ");
    AppBundle::decode(bytes).expect("Failed to decode domino happ")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_holochain::init(
            vec_to_locked(vec![]),
            HolochainPluginConfig::new(holochain_dir(), network_config())
        ))
        .setup(|app| {
            let handle = app.handle().clone();
            let result: anyhow::Result<()> = tauri::async_runtime::block_on(async move {
                setup(handle.clone()).await?;

                // After set up we can be sure our app is installed and up to date, so we can just open it
                let mut window_builder = app.holochain()?
                    .main_window_builder(String::from("main"), false, Some(String::from("domino")), None)
                    .await?;

                #[cfg(not(mobile))]
                {
                    window_builder = window_builder
                        .title(String::from("Domino"))
                        .inner_size(1200.0, 800.0)
                        .menu(
                            Menu::with_items(
                                &handle,
                                &[&Submenu::with_items(
                                    &handle,
                                    "File",
                                    true,
                                    &[
                                        &MenuItem::with_id(
                                           & handle,
                                            "open-logs-folder",
                                            "Open Logs Folder",
                                            true,
                                            None::<&str>,
                                        )?,
                                        &MenuItem::with_id(
                                           & handle,
                                            "factory-reset",
                                            "Factory Reset",
                                            true,
                                            None::<&str>,
                                        )?,
                                        &PredefinedMenuItem::close_window(&handle, None)?,
                                    ],
                                )?],
                            )?
                        )
                        .on_menu_event(|window, menu_event| match menu_event.id().as_ref() {
                            "open-logs-folder" => {
                                let log_folder = window.app_handle()
                                    .path()
                                    .app_log_dir()
                                    .expect("Could not get app log dir");
                                if let Err(err) = tauri_plugin_opener::reveal_item_in_dir(log_folder.clone()) {
                                    log::error!("Failed to open log dir at {log_folder:?}: {err:?}");
                                }
                            }
                            "factory-reset" => {
                                let h = window.app_handle().clone();
                                window.app_handle()
                                        .dialog()
                                        .message("Are you sure you want to perform a factory reset? All your data will be lost.")
                                        .title("Factory Reset")
                                        .buttons(MessageDialogButtons::OkCancel)
                                        .show(move |result| match result {
                                            true => {
                                                if let Err(err) = std::fs::remove_dir_all(holochain_dir()) {
                                                    log::error!("Failed to perform factory reset: {err:?}");
                                                } else {
                                                    h.restart();
                                                }
                                            }
                                            false => {
        
                                            }
                                        });
                            }
                            _ => {}
                        });
                }

                window_builder.build()?;

                Ok(())
            });

            result?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Very simple setup for now:
// - On app start, check whether the app is already installed:
//   - If it's not installed, install it
//   - If it's installed, check if it's necessary to update the coordinators for our hApp,
//     and do so if it is
//
// You can modify this function to suit your needs if they become more complex
async fn setup(handle: AppHandle) -> anyhow::Result<()> {
    let admin_ws = handle.holochain()?.admin_websocket().await?;

    let installed_apps = admin_ws
        .list_apps(None)
        .await
        .map_err(|err| tauri_plugin_holochain::Error::ConductorApiError(err))?;

    if installed_apps.iter().find(|app| app.installed_app_id.as_str().eq(APP_ID)).is_none() {
        handle
            .holochain()?
            .install_app(
                String::from(APP_ID),
                happ_bundle(),
                None,
                None,
                Some(String::from(NETWORK_SEED)),
            )
            .await?;

        Ok(())
    } else {
        handle.holochain()?.update_app_if_necessary(
            String::from(APP_ID),
            happ_bundle()
        ).await?;

        Ok(())
    }
}

fn network_config() -> NetworkConfig {
    let mut network_config = NetworkConfig::default();

    // Don't use the bootstrap service on tauri dev mode
    if tauri::is_dev() {
        network_config.bootstrap_url = url2::Url2::parse("http://0.0.0.0:8888");
    }

    // Don't hold any slice of the DHT in mobile
    if cfg!(mobile) {
        network_config.target_arc_factor = 0;
    }

    network_config
}

fn holochain_dir() -> PathBuf {
    if tauri::is_dev() {
        #[cfg(target_os = "android")]
        {
            app_dirs2::app_root(
                app_dirs2::AppDataType::UserCache,
                &app_dirs2::AppInfo {
                    name: "domino",
                    author: std::env!("CARGO_PKG_AUTHORS"),
                },
            ).expect("Could not get the UserCache directory")
        }
        #[cfg(not(target_os = "android"))]
        {
            let tmp_dir =
                tempdir::TempDir::new("domino").expect("Could not create temporary directory");

            // Convert `tmp_dir` into a `Path`, destroying the `TempDir`
            // without deleting the directory.
            let tmp_path = tmp_dir.into_path();
            tmp_path
        }
    } else {
        app_dirs2::app_root(
            app_dirs2::AppDataType::UserData,
            &app_dirs2::AppInfo {
                name: "domino",
                author: std::env!("CARGO_PKG_AUTHORS"),
            },
        )
        .expect("Could not get app root")
        .join("holochain")
    }
}

