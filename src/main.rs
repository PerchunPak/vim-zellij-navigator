use std::collections::BTreeMap;
use zellij_tile::prelude::*;
use zellij_tile::shim::list_clients;

const CTRL_L: &'static [u8; 1] = b"\x0c";
const CTRL_K: &'static [u8; 1] = b"\x0b";
const CTRL_J: &'static [u8; 1] = b"\x0A";
const CTRL_H: &'static [u8; 1] = b"\x08";

struct State {
    permissions_granted: bool,
    vim_commands: Vec<String>,
    asked_direction: Option<String>,
    print_to_log: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            permissions_granted: false,
            vim_commands: vec!["vim".to_string(), "nvim".to_string()],
            asked_direction: None,
            print_to_log: false,
        }
    }
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
            PermissionType::WriteToStdin,
        ]);
        subscribe(&[EventType::PermissionRequestResult, EventType::ListClients]);
        if self.permissions_granted {
            hide_self();
        }
        self.load_configuration(configuration);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(permission) => {
                self.permissions_granted = match permission {
                    PermissionStatus::Granted => true,
                    PermissionStatus::Denied => false,
                };
                if self.permissions_granted {
                    hide_self();
                }
            }

            Event::ListClients(clients) => {
                if let Some(direction) = self.asked_direction.clone() {
                    self.navigate(direction, clients);
                    self.asked_direction = None;
                } else if self.print_to_log {
                    eprintln!(
                        "[zellij.nvim] got 'ListClients' event but no recorded asked direction"
                    );
                }
            }

            _ => {}
        }
        return false; // No need to render UI.
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if let Some(payload) = pipe_message.payload {
            // I have no idea how to avoid race conditions with Zellij's architecture
            self.asked_direction = Some(payload);
            list_clients();
        } else {
            if self.print_to_log {
                eprintln!("[zellij.nvim] no pipe payload?");
            };
        };

        return false; // No need to render UI.
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}

impl State {
    fn load_configuration(&mut self, configuration: BTreeMap<String, String>) {
        if let Some(vim_commands) = configuration.get("vim_commands") {
            self.vim_commands = vim_commands
                .split('|')
                .map(|s| s.trim().to_string())
                .collect();
        }
        if let Some(print_to_log) = configuration.get("print_to_log") {
            self.print_to_log = matches!(print_to_log.trim(), "true" | "t" | "y" | "1");
        }

        if self.print_to_log {
            eprintln!("[zellij.nvim] Configuration loaded.");
            eprintln!("[zellij.nvim] Vim commands: {:?}", self.vim_commands);
        }
    }

    fn navigate(&mut self, direction: String, clients: Vec<ClientInfo>) {
        if self.print_to_log {
            eprintln!("[zellij.nvim] asked to navigate to '{}'", direction);
        };

        let Some(current_client) = clients
            .iter()
            .find(|client| client.is_current_client && !client.running_command.is_empty())
        else {
            if self.print_to_log {
                eprintln!("[zellij.nvim] no client is running")
            };
            return;
        };

        match current_client.running_command.trim() {
            "N/A" => self.navigate_in_zellij(direction),
            running_command => self.navigate_in_vim(direction, running_command),
        }
    }

    fn navigate_in_vim(&mut self, direction: String, running_command: &str) {
        let running_command_exe = running_command.split_whitespace().collect::<Vec<_>>()[0]
            .split('/')
            .last()
            .unwrap_or("")
            .to_string();

        let is_vim_cmd = self.vim_commands.contains(&running_command.to_string())
            || self.vim_commands.contains(&running_command_exe);

        if self.print_to_log {
            eprintln!(
                "[zellij.nvim] Detected command: `{}`; Executable: `{}`; Is Vim? {}.",
                running_command, running_command_exe, is_vim_cmd,
            );
        };

        if !is_vim_cmd {
            return;
        }

        let bytes = match direction.as_ref() {
            "right" => CTRL_L,
            "right-or-tab" => CTRL_L,
            "up" => CTRL_K,
            "down" => CTRL_J,
            "left" => CTRL_H,
            "left-or-tab" => CTRL_H,
            _ => {
                // no 'if self.print_to_log' because this is a critical mistake
                // there is probably a better way to print a message than to logs
                eprintln!(
                    "[zellij.nvim] invalid direction '{}' provided, should be one of: right, right-or-tab, up, down, left, left-or-tab",
                    direction,
                );
                return;
            }
        };
        write(bytes.to_vec());

        if self.print_to_log {
            eprintln!("[zellij.nvim] sent '{}' key to vim", direction);
        };
    }

    fn navigate_in_zellij(&mut self, direction: String) {
        match direction.as_ref() {
            "right" => move_focus(Direction::Right),
            "right-or-tab" => move_focus_or_tab(Direction::Right),
            "up" => move_focus(Direction::Up),
            "down" => move_focus(Direction::Down),
            "left" => move_focus(Direction::Left),
            "left-or-tab" => move_focus_or_tab(Direction::Left),
            _ => {
                // no 'if self.print_to_log' because this is a critical mistake
                // there is probably a better way to print a message than to logs
                eprintln!(
                    "[zellij.nvim] invalid direction '{}' provided, should be one of: right, right-or-tab, up, down, left, left-or-tab",
                    direction,
                );
                return;
            }
        }

        if self.print_to_log {
            eprintln!("[zellij.nvim] moved to direction '{}'", direction);
        };
    }
}
