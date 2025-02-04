/* main.rs
 *
 * Copyright 2022 Rob Williamson
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use glib::ControlFlow::Continue;
use glib::source::timeout_add_local;
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Application, ApplicationWindow, Box, Frame, Label, Orientation, Widget};

use boiling_frog_dbus::dbus_engine::DbusEngine;
use boiling_frog_dbus::GenericError;

use crate::config::{MARGIN, UPDATE_RATE};

mod config;
mod ui_format;

const APP_ID: &str = "com.robwilliamson.boiling_frog";
const TITLE: &str = "Boiling Frog";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Present window
    match build_happy_path_ui(app) {
        Ok(window) => window.present(),
        Err(e) => {
            println!("{}", e.to_string());
            build_error_path_ui(app).present()
        }
    }
}

fn build_error_path_ui(app: &Application) -> ApplicationWindow {
    let grid = set_margins!(Box::builder(), MARGIN)
        .orientation(Orientation::Vertical)
        .build();
    let label = set_margins!(Label::builder(), MARGIN)
        .label("Could not receive device thermal data. Is Hardware Sensors Indicator installed?")
        .selectable(true)
        .build();
    let link = set_margins!(Label::builder(), MARGIN)
        .use_markup(true)
        .label("<a href=\"https://github.com/alexmurray/indicator-sensors\">Hardware Sensors Indicator</a>")
        .selectable(true)
        .build();

    grid.append(&label);
    grid.append(&link);

    make_window(app, &grid)
}

fn build_happy_path_ui(app: &Application) -> Result<ApplicationWindow, GenericError> {
    let engine = DbusEngine::new()?;

    // Create Label
    let temperature_title_label = set_margins!(Label::builder(), MARGIN)
        .label("Maximum Temperature")
        .build();

    let temperature_value_label = set_margins!(Label::builder(), MARGIN)
        .use_markup(true)
        .label(make_value_units_string!(&engine.temp()))
        .build();

    // https://docs.gtk.org/gtk4/visual_index.html
    let temperature_grid = set_margins!(Box::builder(), MARGIN)
        .orientation(Orientation::Vertical)
        .build();

    temperature_grid.append(&temperature_title_label);
    temperature_grid.append(&temperature_value_label);

    let temperature_frame = set_margins!(Frame::builder(), MARGIN)
        .child(&temperature_grid)
        .build();

    // Create Label
    let fan_title = set_margins!(Label::builder(), MARGIN)
        .label("Highest Fan Speed")
        .build();

    // https://docs.gtk.org/Pango/pango_markup.html
    let fan_speed = set_margins!(Label::builder(), MARGIN)
        .use_markup(true)
        .label(make_value_units_string!(&engine.fan()))
        .build();

    // https://docs.gtk.org/gtk4/visual_index.html
    let fan_grid = set_margins!(Box::builder(), MARGIN)
        .orientation(Vertical)
        .build();

    fan_grid.append(&fan_title);
    fan_grid.append(&fan_speed);

    let fan_frame = set_margins!(Frame::builder(), MARGIN)
        .child(&fan_grid)
        .build();

    let metrics_grid = set_margins!(Box::builder(), MARGIN)
        .orientation(Horizontal)
        .build();

    metrics_grid.append(&temperature_frame);
    metrics_grid.append(&fan_frame);

    let gtk_box = Box::builder().orientation(Vertical).build();
    gtk_box.append(&metrics_grid);

    // Poll the engine because GTK is not thread-safe.
    timeout_add_local(UPDATE_RATE, move || {
        fan_speed.set_label(&make_value_units_string!(&engine.fan()));
        temperature_value_label.set_label(&make_value_units_string!(&engine.temp()));
        Continue
    });

    Ok(make_window(app, &gtk_box))
}

fn make_window(app: &Application, child: &impl IsA<Widget>) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .title(TITLE)
        .child(child)
        .build()
}
