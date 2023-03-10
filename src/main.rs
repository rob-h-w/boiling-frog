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

use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Application, ApplicationWindow, Box, Frame, Label, Orientation};

use crate::config::MARGIN;

mod config;
mod ui_format;

const APP_ID: &str = "com.robwilliamson.boiling_frog";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create Label
    let temperature_title_label = set_margins!(Label::builder(), MARGIN)
        .label("Maximum Temperature")
        .build();

    // https://docs.gtk.org/Pango/pango_markup.html
    let temperature_value_label = set_margins!(Label::builder(), MARGIN)
        .use_markup(true)
        .label("<span font_size='40000'>0C</span>")
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
        .label("<span font_size='40000'>0RPM</span>")
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

    // Add buttons to `gtk_box`
    let gtk_box = Box::builder().orientation(Vertical).build();
    gtk_box.append(&metrics_grid);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .resizable(false)
        .build();

    // Present window
    window.present();
}
