# Site Builder

This application's primary function is to generate a portfolio website.

For directions on how to use this see [tutorial.md](./tutorial/tutorial.md)

## Technical Overview
The basic idea is that the user will add a series of projects, each project has a title, subtitle, a description, a list of images and an optional list of contributors. The information will be stored in the `source` directory with the following structure.
```
[source]
   ├───fonts
   │     ├─── [bold]
   │     └─── [normal]
   ├─── portfolio
   │     └─── [project folder] (repeated)
   │           ├─── img
   │                └───[image file] (repeated)
   │           ├─── content.md
   │           └─── meta.toml
   ├─── .site-builder
   ├─── about.md
   └─── [image file]
```
- `fonts`
  - Two fonts must be provided
- `portfolio`
  - A folder will be created here for each project added to a site
- `[project folder]`
  - Each of these folders will have the same title as a project, initial values are set as 'project-#' where # is the new length of the list of projects
- `img`
  - This will contain any images the user has added to a project
- `content.md`
  - This will be any content the user has entered as a project's description
  - The user can enter any valid markdown and it will be converted to the corresponding html
- `meta.toml`
  - This file will contain some supplemental information about a project
    - Title
    - Subtitle (context)
    - Contributors (teammates)
  - This file will be written in the `TOML` format
- `.site-builder`
  - This file will include some additional metadata about our site
    - The input/output paths
    - The absolute path's of all of the included files
    - The order of the images for each project
      - This is important because the 1st image will always be what is displayed on the main page
  - The format the information is stored in is a binary format called [`bincode`](https://github.com/TyOverby/bincode)
- `about.md`
  - This will contain the information entered by the user that should appear on the 'About' page
- `[image file]`
  - This will be the image that should appear on the 'About' page

When a user clicks the 'Generate' button it will convert the files in the `source` folder to a valid website in the `destination` folder. This will consist of the following structure.

```
[destination]
   ├─── about
   │     └─── index.html
   ├─── contact
   │     └─── index.html
   ├─── fonts
   │     ├─── [bold]
   │     └─── [normal]
   ├─── portfolio
   │     └─── [project folder] (repeated)
   │           ├─── img
   │           │     └─── [image file] (repeated)
   │           └─── index.html
   ├─── index.html
   └─── [image file]
```

The example html output can be found [here](./docs).