use futures_util::stream::StreamExt;
use shiplift::{rep::Image, BuildOptions, ContainerOptions, Docker, ImageListOptions};

use crate::error::Error;
use crate::simulator::Simulator;
use std::collections::{HashMap, HashSet};

pub async fn add_simulator(sim: &Simulator, path: &String) -> Option<Error> {
    let docker = Docker::new();

    let image_tag = format!("meta/{}:{}", sim.name(), sim.version());

    match find_image(&docker, &image_tag).await {
        Ok(None) => return Some(Error::ImageError(String::from("Simulator already exists"))),
        Ok(Some(_)) => (),
        Err(error) => return Some(Error::DockerError(error.to_string())),
    };

    let mut image_res = docker
        .images()
        .build(&BuildOptions::builder(path).tag(image_tag).build());

    while let Some(item) = image_res.next().await {
        match item {
            Ok(val) => println!("{}", val),
            Err(error) => {
                eprintln!("An error has occurred: {}", error);
                return Some(Error::DockerError(error.to_string()));
            }
        };
    }

    None
}

async fn find_image(docker: &Docker, image_tag: &String) -> Result<Option<Image>, shiplift::Error> {
    match docker
        .images()
        .list(&ImageListOptions::builder().filter_name(&image_tag).build())
        .await
    {
        Ok(images) => Ok(images.get(0).cloned()),
        Err(error) => Err(error),
    }
}

pub async fn remove_simulator(sim: &Simulator) -> Option<Error> {
    let docker = Docker::new();

    let image_tag = format!("meta/{}:{}", sim.name(), sim.version());

    match docker.images().get(image_tag).delete().await {
        Err(error) => Some(Error::DockerError(error.to_string())),
        Ok(_) => None,
    }
}

pub async fn list_simulators() -> Option<Error> {
    list_simulators_with_filter(&String::new()).await
}

pub async fn list_simulators_with_filter(filter: &String) -> Option<Error> {
    let docker = Docker::new();

    match docker
        .images()
        .list(&ImageListOptions::builder().filter_name(filter).build())
        .await
    {
        Ok(images) => {
            print_images(images);
            None
        }
        Err(error) => Some(Error::DockerError(error.to_string())),
    }
}

fn print_images(images: Vec<Image>) {
    let tags: HashSet<&str> = images
        .iter()
        .filter_map(|image| image.repo_tags.as_ref())
        .map(|tags| -> Vec<&String> {
            tags.iter()
                .filter(|&tag| tag.starts_with("meta/"))
                .collect()
        })
        .map(|tags| -> Vec<&str> {
            tags.iter()
                .map(|&tag| tag.strip_prefix("meta/").unwrap())
                .collect()
        })
        .flatten()
        .collect();

    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

    tags.iter()
        .map(|&tag| tag.split(":").collect())
        .for_each(|split: Vec<_>| {
            let entry = hash_map
                .entry(split.get(0).unwrap().to_string())
                .or_insert(Vec::new());
            entry.push(split.get(1).unwrap().to_string());
        });

    for (key, value) in hash_map {
        println!("{}:", key);
        for version in value {
            println!("* {}", version);
        }
        println!()
    }
}

pub async fn run_simulator(sim: Simulator) -> Option<Error> {
    let docker = Docker::new();

    let image = find_image(&docker, &sim.tag()).await;

    match image {
        Ok(image) => match image {
            Some(_) => {
                match docker
                    .containers()
                    .create(
                        &ContainerOptions::builder(&sim.name())
                            .expose(3000, "TCP", 3000)
                            .build(),
                    )
                    .await
                {
                    Ok(_) => (),
                    Err(error) => return Some(Error::DockerError(error.to_string())),
                }
            }
            None => return Some(Error::ImageError(String::from("Image does not exist"))),
        },
        Err(error) => return Some(Error::DockerError(error.to_string())),
    };

    None
}
