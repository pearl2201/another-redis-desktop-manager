use egui::Window;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NewConnectionWindow {
    // Example stuff:
    host: String,
    port: i32,
    password: String,
    username: String,
    connection_name: String,
    separator: String,
    ssh: bool,
    ssl: bool,
    sentinel: bool,
    cluster: bool,
    readonly: bool,
    ssh_host: String,
    ssh_port: i32,
    ssh_password: String,
    ssh_username: String,
    ssh_private_key_path: String,
    ssh_timeout: i32,
    ssh_passphrase: String,

    ssl_private_key_path: String,
    ssl_public_key_path: String,
    ssl_authority_key_path: String,
    sentinel_redis_node_password: String,
    sentinel_master_group_name: String,

    show: bool,
}

impl Default for NewConnectionWindow {
    fn default() -> Self {
        Self {
            // Example stuff:
            host: "".to_string(),
            port: 6379,
            password: "".to_string(),
            username: "".to_string(),
            connection_name: "".to_string(),
            separator: "".to_string(),
            ssh: false,
            ssl: false,
            sentinel: false,
            cluster: false,
            readonly: false,
            ssh_host: "".to_string(),
            ssh_port: 22,
            ssh_password: "".to_string(),
            ssh_username: "".to_string(),
            ssh_private_key_path: "".to_string(),
            ssh_timeout: 30,
            ssh_passphrase: "".to_string(),

            ssl_private_key_path: "".to_string(),
            ssl_public_key_path: "".to_string(),
            ssl_authority_key_path: "".to_string(),
            sentinel_redis_node_password: "".to_string(),
            sentinel_master_group_name: "".to_string(),
            show: false,
        }
    }
}

impl NewConnectionWindow {
    pub fn draw(&mut self, ctx: &egui::Context) {
        Window::new("New connection")
            .open(&mut self.show)
            .show(ctx, |ui| {
                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Host");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.host).hint_text("127.0.0.1"),
                        );
                    });

                    ui.vertical(|ui| {
                        ui.label("Port");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.port).hint_text("6379"),
                        );
                    });
                    ui.end_row();

                    ui.label("Second row, first column");
                    ui.label("Second row, second column");
                    ui.label("Second row, third column");
                    ui.end_row();

                    ui.horizontal(|ui| {
                        ui.label("Same");
                        ui.label("cell");
                    });
                    ui.label("Third row, second column");
                    ui.end_row();

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.ssh, "SSH");
                    });
                    ui.end_row();
                });
            });
    }

    pub fn showWindow(&mut self) {
        self.show = true;
    }
}
