



#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use sea_orm::{Database, TransactionTrait};
    use sea_orm::prelude::Uuid;
    use tracing::{info, Level};
    use tracing_subscriber::fmt;
    use cerium::client::driver::web::WebDriver;

    use engine::controller::case::CaseController;

    #[tokio::test]
    async fn dry_run_test_controller() {
        // init_logger();
        fmt().with_max_level(Level::DEBUG).init();
        let case_id = "731453aa-95a5-4180-be0d-c211a1e92aad".to_string();

        let uri = Some("postgres://root:root@localhost:5432/orca".to_string());

        let db = Database::connect(uri.unwrap()).await.expect("Error unable to connect DB");
        let trx = db.begin().await.expect("Error unable to connect DB");
        info!("got the db");
        let ui_driver = WebDriver::default().await.expect("error");
        info!("got the driver");
        let controller = CaseController::new(&trx, ui_driver.clone());
        info!("got the controller");
        controller.run_case(Uuid::from_str(case_id.as_str()).expect("")).await.expect("error");
        ui_driver.driver.quit().await;
    }

    // #[test]
    // fn serialize_test() {
    //     let data = json!(
    //     [
    //       {
    //         "description": "Click on Submit",
    //         "type": "enter",
    //         "target": {
    //           "type": "id",
    //           "value": "email"
    //         },
    //         "data": {
    //           "type": "static",
    //           "value": "mani@gmail.com"
    //         }
    //       },
    //       {
    //         "description": "Update user name",
    //         "type": "enter",
    //         "target": {
    //           "type": "id",
    //           "value": "password"
    //         },
    //         "data": {
    //           "type": "runtime",
    //           "value": "UserPWD"
    //         }
    //       },
    //       {
    //         "description": "login",
    //         "type": "click",
    //         "target": {
    //           "type": "id",
    //           "value": "button"
    //         }
    //       }
    //     ]);
    //     let action: Vec<Action> = serde_json::from_value(data).expect("Panic from the Test case");
    //     let action_group = ActionGroup {
    //         actions: action,
    //         name: "Login testing for the automation".to_string(),
    //         description: None,
    //     };
    //
    //     let mut caps = DesiredCapabilities::firefox();
    //     caps.set_headless().expect("TODO: panic message");
    //
    //     let d = WebDriver::new("https://localhost:4444//wd/hub/session", caps);
    //     let driver = Runtime::new().unwrap().block_on(d).expect("TODO: panic message");
    //     let d = UIDriver::default(&driver);
    //     d.execute(&action_group).expect("TODO: panic message");
    //
    //     println!("{:?}", action_group)
    // }
}
