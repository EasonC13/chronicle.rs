// import the apps you want to build
use chronicle_storage::storage::storage::StorageBuilder;
use chronicle_api::api::api::ApiBuilder;
// import launcher macro
use chronicle_common::launcher;

// create event type
enum Event {
    StartApp(String),
    ShutdownApp(String),
    AknShutdown(String),
    RegisterApp(String, Box<dyn ShutdownTx>),
    RegisterDashboard(String, Box<dyn DashboardTx>),
    AppsStatus(String),
}

launcher!(
    apps_builder: AppsBuilder {storage: StorageBuilder, api: ApiBuilder }, // Apps
    apps: Apps{}, // Launcher state
    event: Event // Launcher event type
);

// required implemenetation
impl LauncherEvent for Event {
    fn start_app(app_name: String) -> Event {
        Event::StartApp(app_name)
    }
    fn apps_status(dashboard_name: String) -> Event {
        Event::AppsStatus(dashboard_name)
    }
    fn shutdown_app(app_name: String) -> Event {
        Event::ShutdownApp(app_name)
    }
    fn aknowledge_shutdown(app_name: String) -> Event {
        Event::AknShutdown(app_name)
    }
    fn register_dashboard(dashboard_name: String, dashboard_tx: Box<dyn DashboardTx>) -> Event {
        Event::RegisterDashboard(dashboard_name, dashboard_tx)
    }
    fn register_app(app_name: String, shutdown_tx: Box<dyn ShutdownTx>) -> Event {
        Event::RegisterApp(app_name, shutdown_tx)
    }
}

// build your apps
impl AppsBuilder {
    fn build(self) -> Apps {
        // - storage app:
        let storage = StorageBuilder::new()
        .listen_address("0.0.0.0:8080".to_string())
        .thread_count(2)
        .local_dc("datacenter1".to_string())
        .reporter_count(1)
        .buffer_size(1024000)
        .recv_buffer_size(1024000)
        .send_buffer_size(1024000)
        .nodes(vec!["172.17.0.2:9042".to_string()]);
        // - build api app
        // TODO
        // add app to AppsBuilder then transform it to Apps
        self.storage(storage)
        .to_apps()
    }
}
// launcher event loop
impl Apps {
    async fn run(mut self) {
        while let Some(event) = self.rx.0.recv().await {
            match event {
                _ => {

                }
            }
        };
    }
}

#[tokio::main(core_threads = 8)]
async fn main() {
    println!("starting chronicle-example");
    AppsBuilder::new()
    .build() // build apps first
    .storage().await // start first app
    .run().await; // launcher event loop
}