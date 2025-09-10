mod menu;
mod utils;
use holochain_types::prelude::AppBundle;
use std::path::PathBuf;
use tauri_plugin_holochain::{vec_to_locked, HolochainExt, HolochainPluginConfig, NetworkConfig};
// use tauri_plugin_opener::OpenerExt;
use anyhow::anyhow;
#[cfg(not(mobile))]
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
// #[cfg(all(desktop))]
// use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

#[cfg(not(mobile))]
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
mod tauri_config_reader;
use tauri_config_reader::AppConfig;
use holochain_client::AppStatusFilter;

const FOR_HOLOCHAIN_DIR: &'static str = "co.unyt.unyt.sandbox";

// const DNA_HASH: &'static str = "unyt-dna_hashes";
// const DNA_HASH: &'static str = include_str!("../../workdir/dash-chat-dna_hashes");

pub fn happ_bundle() -> AppBundle {
    let bytes = include_bytes!("../../workdir/unyt.happ");
    AppBundle::decode(bytes).expect(&"Failed to decode unyt.happ")
}

macro_rules! log {
    ($($arg:tt)*) => {
        println!("🟦 {}", format!($($arg)*))
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Enhanced environment variables for debugging
    // std::env::set_var("WASM_LOG", "debug");
    // std::env::set_var("RUST_LOG", "debug"); // Add general Rust logging
    
    // Log startup information
    log!("🚀 Starting Unyt application...");
    log!("App version: {}", env!("CARGO_PKG_VERSION"));
    log!("Build mode: {}", if tauri::is_dev() { "development" } else { "production" });

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                // Configure explicit targets and higher log levels for production
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir { file_name: Some("unyt".to_string()) }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(if tauri::is_dev() {
                    log::LevelFilter::Debug  // More verbose in dev
                } else {
                    // todo: check to Info when done debugging  
                    log::LevelFilter::Info   // More verbose in production for debugging
                })
                .level_for("tauri", log::LevelFilter::Warn)
                .level_for("unyt", log::LevelFilter::Warn)
                .level_for("unyt_tauri", log::LevelFilter::Warn)
                .level_for("tauri_app_lib", log::LevelFilter::Warn)
                .level_for("tracing::span", log::LevelFilter::Off)
                .level_for("iroh", log::LevelFilter::Warn)
                .level_for("holochain", log::LevelFilter::Debug)
                .level_for("kitsune2", log::LevelFilter::Warn)
                .level_for("kitsune2_gossip", log::LevelFilter::Warn)
                .level_for("holochain_runtime", log::LevelFilter::Info)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_holochain::init(
            vec_to_locked(vec![]), // TODO: path to add password on startup
            HolochainPluginConfig::new(holochain_dir(), network_config())
        ))
        .setup(|app| {
            log!("🔧 Starting application setup...");
            let handle = app.handle().clone();
            
            // NOW we can log - logger is initialized
            log!("App config: {:?}", AppConfig::new(handle.clone()));
            log!("Holochain directory: {:?}", holochain_dir());
            
            // Log file locations for easy debugging
            if let Ok(log_dir) = handle.path().app_log_dir() {
                log!("📁 Log files location: {:?}", log_dir);
            }
            
            let result: anyhow::Result<()> = tauri::async_runtime::block_on(async move {
                log!("📦 Calling setup function...");
                setup(handle.clone()).await?;
                log!("✅ Setup completed successfully");

                // After set up we can be sure our app is installed and up to date, so we can just open it
                log!("🪟 Creating main window...");
                let mut window_builder = app.holochain()?
                .main_window_builder(String::from("main"), false, Some(AppConfig::new(handle.clone()).app_id), None)
                .await?;
               #[cfg(not(mobile))]
                {
                    window_builder = window_builder
                        .title(String::from(AppConfig::new(handle.clone()).product_name))
                        .inner_size(1200.0, 800.0)
                        .menu(
                            Menu::with_items(
                                &handle,
                                &[
                                    &Submenu::with_items(
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
                                            // &MenuItem::with_id(
                                            //    & handle,
                                            //     "factory-reset",
                                            //     "Factory Reset",
                                            //     true,
                                            //     None::<&str>,
                                            // )?,
                                            &PredefinedMenuItem::close_window(&handle, None)?,
                                        ],
                                    )?,
                                    &Submenu::with_items(
                                        &handle,
                                        "Help",
                                        true,
                                        &[&MenuItem::with_id(
                                            &handle,
                                            "about",
                                            "About",
                                            true,
                                            None::<&str>,
                                        )?],
                                    )?,
                                ],
                            )?
                        )
                        .on_menu_event(move |window, menu_event| match menu_event.id().as_ref() {
                            "open-logs-folder" => {
                                let log_folder = window.app_handle()
                                    .path()
                                    .app_log_dir()
                                    .expect("Could not get app log dir");
                                if let Err(err) = tauri_plugin_opener::reveal_item_in_dir(log_folder.clone()) {
                                    log!("Failed to open log dir at {log_folder:?}: {err:?}");
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
                                                    log!("Failed to perform factory reset: {err:?}");
                                                } else {
                                                    h.restart();
                                                }
                                            }
                                            false => {
                                                log!("Factory reset cancelled");
                                            }
                                        });
                            }
                            "about" => {
                                let h = window.app_handle().clone();
                                tauri::async_runtime::spawn(async move {
                                    menu::about_menu(&h).await
                                });
                            }
                            _ => {}
                        });
                }

                log!("🎯 Building window...");
                window_builder.build()?;
                log!("✅ Window built successfully");

                // // Setup system tray
                // #[cfg(all(desktop))]
                // setup_tray(&handle)?;

                Ok(())
            });

            match &result {
                Ok(_) => log!("🎉 Application startup completed successfully"),
                Err(e) => log!("❌ Application startup failed: {:?}", e),
            }

            result?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Enhanced setup function with detailed logging
async fn setup(handle: AppHandle) -> anyhow::Result<()> {
    log!("🔌 Connecting to Holochain admin websocket...");
    let admin_ws = handle.holochain()?.admin_websocket().await?;
    
    log!("📋 Listing installed apps...");
    let installed_apps = admin_ws
        .list_apps(Some(AppStatusFilter::Running))
        .await
        .map_err(|err| {
            log!("Failed to list apps: {:?}", err);
            tauri_plugin_holochain::Error::ConductorApiError(err)
        })?;
    
    log!("Found {} installed apps", installed_apps.len());
    for app in &installed_apps {
        log!("Installed app: {} (status: {:?})", app.installed_app_id, app.status);
    }

    let app_config = AppConfig::new(handle.clone());
    let app_is_already_installed = installed_apps
        .iter()
        .find(|app| {
            app.installed_app_id
                .as_str()
                .eq(&app_config.app_id)
        })
        .is_some();

    log!("App '{}' already installed: {}", app_config.app_id, app_is_already_installed);

    if !app_is_already_installed {
        log!("🔍 Looking for previous app versions to migrate...");
        let previous_app = installed_apps
            .iter()
            .filter(|app| {
                app.installed_app_id
                    .as_str()
                    // check for the app prefix
                    .starts_with(app_config.identifier.as_str())
            })
            .min_by_key(|app_info| app_info.installed_at);

        if let Some(previous_app) = previous_app {
            log!("🔄 Migrating from previous app: {}", previous_app.installed_app_id);
            utils::migrate_app(
                &handle.holochain()?.holochain_runtime,
                previous_app.installed_app_id.clone(),
                app_config.app_id.clone(),
                happ_bundle(),
                None,
                Some(app_config.network_seed.clone()),
            )
            .await?;

            log!("🔌 Disabling previous app: {}", previous_app.installed_app_id);
            admin_ws
                .disable_app(previous_app.installed_app_id.clone())
                .await
                .map_err(|err| {
                    log!("Failed to disable previous app: {:?}", err);
                    anyhow!("{err:?}")
                })?;
        } else {
            log!("📥 Installing new app: {}", app_config.app_id);
            handle
                .holochain()?
                .install_app(
                    app_config.app_id.clone(),
                    happ_bundle(),
                    None,
                    None,
                    Some(app_config.network_seed.clone()),
                )
                .await?;
        }

        log!("✅ App installation/migration completed");
        Ok(())
    } else {
        log!("🔄 Checking if app needs update...");
        handle
            .holochain()?
            .update_app_if_necessary(
                app_config.app_id.clone(),
                happ_bundle(),
            )
            .await?;

        log!("✅ App update check completed");
        Ok(())
    }
}

// #[cfg(all(desktop))]
// fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
//     let tray_menu = Menu::with_items(
//         app,
//         &[
//             &MenuItem::with_id(app, "show", "Show", true, None::<&str>)?,
//             &MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?,
//             &PredefinedMenuItem::separator(app)?,
//             &MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?,
//         ],
//     )?;

//     let _tray = TrayIconBuilder::new()
//         .icon(app.default_window_icon().unwrap().clone())
//         .menu(&tray_menu)
//         .menu_on_left_click(false)
//         .tooltip("Unyt")
//         .on_menu_event(move |app, event| match event.id().as_ref() {
//             "show" => {
//                 if let Some(window) = app.get_webview_window("main") {
//                     let _ = window.show();
//                     let _ = window.set_focus();
//                 }
//             }
//             "hide" => {
//                 if let Some(window) = app.get_webview_window("main") {
//                     let _ = window.hide();
//                 }
//             }
//             "quit" => {
//                 app.exit(0);
//             }
//             _ => {}
//         })
//         .on_tray_icon_event(|tray, event| match event {
//             TrayIconEvent::Click {
//                 button: tauri::tray::MouseButton::Left,
//                 button_state: tauri::tray::MouseButtonState::Up,
//                 ..
//             } => {
//                 let app = tray.app_handle();
//                 if let Some(window) = app.get_webview_window("main") {
//                     let _ = if window.is_visible().unwrap_or(false) {
//                         window.hide()
//                     } else {
//                         window.show();
//                         window.set_focus()
//                     };
//                 }
//             }
//             _ => {}
//         })
//         .build(app)?;

//     Ok(())
// }

fn network_config() -> NetworkConfig {
    let mut network_config = NetworkConfig::default();

    // Don't use the bootstrap service on tauri dev mode
    // if tauri::is_dev() {
    //     network_config.bootstrap_url = url2::Url2::parse("http://0.0.0.0:8888");
    // } else {
        network_config.bootstrap_url =
            url2::Url2::parse("https://dev-test-bootstrap2.holochain.org/");
    // }

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
                    name: FOR_HOLOCHAIN_DIR,
                    author: std::env!("CARGO_PKG_AUTHORS"),
                },
            )
            .expect("Could not get the UserCache directory")
        }
        #[cfg(not(target_os = "android"))]
        {
            let tmp_dir = tempdir::TempDir::new(FOR_HOLOCHAIN_DIR)
                .expect("Could not create temporary directory");

            // Convert `tmp_dir` into a `Path`, destroying the `TempDir`
            // without deleting the directory.
            let tmp_path = tmp_dir.into_path();
            tmp_path
        }
    } else {
        app_dirs2::app_root(
            app_dirs2::AppDataType::UserData,
            &app_dirs2::AppInfo {
                name: FOR_HOLOCHAIN_DIR,
                author: std::env!("CARGO_PKG_AUTHORS"),
            },
        )
        .expect("Could not get app root")
        .join("holochain")
    }
}
