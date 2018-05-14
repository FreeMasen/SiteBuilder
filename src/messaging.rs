use std::path::PathBuf;
use project::Project;
use state::Route;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A message for the client to display
pub struct ServerMessage {
    pub id: u32,
    pub content: String,
    pub is_error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "kind")]
/// The message received from the client regarding the 
/// required changes to the state
pub enum Message {
    /// Window loaded
    Load, 
    /// React app initialized
    Init, 
    // refresh state from file system
    Refresh, 
    /// report client error
    Error { message: String },
    /// build the site
    Build, 
    /// add a new project
    AddProject { name: String }, 
    /// update a project
    UpdateProject { project: Project },
    /// update the about section text
    UpdateAbout { content: String },
    /// update the about image
    UpdateAboutImage,
    /// Client logging
    Log { msg: String },
    /// Update the source
    UpdateSource,
    /// Update the destination
    UpdateDest,
    /// Add a project image
    AddProjectImage,
    /// remove a project image
    RemoveProjectImage { path: PathBuf},
    /// Change the current view
    ChangeView { route: Route, project: Option<Project> },
    /// Add a font file
    AddFont { bold: bool },
    /// Remove a font file
    RemoveFont { bold: bool },
    /// Delete the selected project
    DeleteProject,
    /// Clear the message after a js setTimeout
    ClearMessage { id: u32 },
    /// Select a site
    ChooseSite { idx: usize },
    /// Change the site's title
    ChangeSiteTitle { title: String },
    /// Add a new site
    AddSite,
}
