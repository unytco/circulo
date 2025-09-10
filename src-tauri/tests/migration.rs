// use profiles_provider_zome_trait::Profile;
// use tauri_app_lib::{happ_bundle, migrate_app};
// use tauri_plugin_holochain::{
//     vec_to_locked, AllowedOrigins, AppBundle, ExternIO, HolochainRuntime, HolochainRuntimeConfig,
//     NetworkConfig,
// };
// use tempdir::TempDir;

// pub fn old_happ_bundle() -> AppBundle {
//     let bytes = include_bytes!("../../workdir/old-unyt.happ");
//     AppBundle::decode(bytes).expect("Failed to decode unyt happ")
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn migrate_app_test() {
//     let tmp = TempDir::new("unyt").unwrap();

//     let runtime = HolochainRuntime::launch(
//         vec_to_locked(vec![]),
//         HolochainRuntimeConfig {
//             holochain_dir: tmp.path().to_path_buf(),
//             network_config: NetworkConfig::default(),
//             admin_port: None,
//         },
//     )
//     .await
//     .unwrap();

//     let old_app_id = String::from("app1");
//     let new_app_id = String::from("app2");

//     runtime
//         .install_app(old_app_id.clone(), old_happ_bundle(), None, None, None)
//         .await
//         .unwrap();

//     let app_ws = runtime
//         .app_websocket(old_app_id.clone(), AllowedOrigins::Any)
//         .await
//         .unwrap();

//     app_ws
//         .call_zome(
//             ZomeCallTarget::RoleName("main".into()),
//             "friends".into(),
//             "set_my_profile".into(),
//             ExternIO::encode(Profile {
//                 name: "alice".into(),
//                 avatar: None,
//                 fields: Default::default(),
//             })
//             .unwrap(),
//         )
//         .await
//         .unwrap();

//     migrate_app(
//         &runtime,
//         old_app_id,
//         new_app_id.clone(),
//         happ_bundle(),
//         None,
//     )
//     .await
//     .unwrap();

//     let app_ws = runtime
//         .app_websocket(new_app_id.clone(), AllowedOrigins::Any)
//         .await
//         .unwrap();

//     let profile: Option<Profile> = app_ws
//         .call_zome(
//             ZomeCallTarget::RoleName("main".into()),
//             "friends".into(),
//             "get_profile".into(),
//             ExternIO::encode(app_ws.my_pub_key.clone()).unwrap(),
//         )
//         .await
//         .unwrap()
//         .decode()
//         .unwrap();

//     assert!(profile.is_some());
//     assert_eq!(profile.unwrap().name, String::from("alice"))
// }
