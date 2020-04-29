#[macro_export]
macro_rules! launcher {
    (
        apps_builder: $name:ident {$($app:ident : $t:ty),+},
        apps: $apps:ident {$($field:ident : $type:ty),*},
        event: $event:ty
    ) => {
        use tokio::sync::mpsc;
        use chronicle_common::traits::{
            launcher::{
                LauncherTx,
            },
            shutdown::ShutdownTx,
            dashboard::DashboardTx,
        };
        pub trait LauncherEvent: Send {
            fn start_app(app_name: String) -> Self;
            fn shutdown_app(app_name: String) -> Self;
            fn aknowledge_shutdown(app_name: String) -> Self;
            fn register_app(app_name: String, shutdown_tx: Box<dyn ShutdownTx>) -> Self;
            fn register_dashboard(dashboard_name: String, dashboard_tx: Box<dyn DashboardTx>) -> Self;
            fn apps_status(dashboard_name: String) -> Self;
        }
        #[derive(Clone)]
        pub struct Sender(mpsc::UnboundedSender<$event>);
        pub struct Receiver(mpsc::UnboundedReceiver<$event>);
        impl LauncherTx for Sender {
            fn start_app(&mut self, app_name: String) {
                let _ = self.0.send(LauncherEvent::start_app(app_name));
            }
            fn shutdown_app(&mut self, app_name: String) {
                let _ = self.0.send(LauncherEvent::shutdown_app(app_name));
            }
            fn aknowledge_shutdown(&mut self, app_name: String) {
                let _ = self.0.send(LauncherEvent::aknowledge_shutdown(app_name));
            }
            fn register_dashboard(&mut self, dashboard_name: String, dashboard_tx: Box<dyn DashboardTx>) {
                let _ = self.0.send(LauncherEvent::register_dashboard(dashboard_name, dashboard_tx));
            }
            fn register_app(&mut self, app_name: String, shutdown_tx: Box<dyn ShutdownTx>) {
                let _ = self.0.send(LauncherEvent::register_app(app_name, shutdown_tx));
            }
            fn apps_status(&mut self, dashboard_name: String) {
                let _ = self.0.send(LauncherEvent::apps_status(dashboard_name));
            }
        }
        #[derive(Default)]
        pub struct $name {
            tx: Option<Sender>,
            rx: Option<Receiver>,
            $(
                $app: Option<$t>,
            )*
        }
        pub struct $apps {
            app_count: usize,
            tx: Sender,
            rx: Receiver,
            $(
                $app: Option<$t>,
            )*
            $(
                $field: Option<$type>,
            )*
        }
        impl $apps {
            $(
                async fn $app(mut self) -> Self {
                    self.$app.take().unwrap().build().run().await;
                    self
                }
            )*
        }
        impl $name {
            pub fn new() -> Self {
                let (tx, rx) = mpsc::unbounded_channel::<$event>();
                let mut launcher = Self::default();
                launcher.tx.replace(Sender(tx));
                launcher.rx.replace(Receiver(rx));
                launcher
            }

            pub fn clone_tx(&self) -> Sender {
                self.tx.as_ref().unwrap().clone()
            }

            pub fn to_apps(mut self) -> $apps {
                $apps {
                    app_count: self.app_count(),
                    tx: self.tx.take().unwrap(),
                    rx: self.rx.take().unwrap(),
                    $(
                        $app: self.$app,
                    )*
                }
            }
            $(
                pub fn $app(mut self, $app: $t) -> Self {
                    self.$app.replace($app.launcher_tx(Box::new(self.clone_tx())));
                    self
                }
            )*

            fn app_count(&self) -> usize {
                launcher!(@count $($app),+)
            }
        }

    };
    (@count $t1:tt, $($t:tt),+) => { 1 + launcher!(@count $($t),+) };
    (@count $t:tt) => { 1 };
}
