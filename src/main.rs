use plotters::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use csv::Reader;
use serde::Deserialize;
use crate::graph::Graph;

mod graph;
#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize)]
struct NetflixEntry {
    show_id: Option<String>,
    title: Option<String>,
    director: Option<String>,
    cast: Option<String>,
    country: Option<String>,
    date_added: Option<String>,
    release_year: Option<String>,
    rating: Option<String>,
    duration: Option<String>,
    listed_in: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("netflix.csv")?;
    let mut csv_reader = Reader::from_reader(file);
    let mut graph = Graph::new();
    let mut movie_titles: HashMap<String, usize> = HashMap::new();
    let mut genre_movies: HashMap<String, HashSet<String>> = HashMap::new();
    let mut total_movie_counts: HashMap<String, usize> = HashMap::new();
    let mut horror_movie_counts: HashMap<String, usize> = HashMap::new();

    for result in csv_reader.deserialize::<NetflixEntry>() {
        let record: NetflixEntry = result?;
        if let Some(title) = record.title {
            movie_titles.entry(title.clone()).and_modify(|c| *c += 1).or_insert(1); 
            
            graph.add_vertex(&title);
            
            if let Some(listed_in) = record.listed_in {
                let genres: Vec<&str> = listed_in.split(',').map(|s| s.trim()).collect();
                for genre in genres {
                    genre_movies.entry(genre.to_string()).or_insert(HashSet::new()).insert(title.clone());
                }
                total_movie_counts.entry(listed_in.clone()).and_modify(|c| *c += 1).or_insert(1);
                if listed_in.contains("Horror") {
                    horror_movie_counts.entry(listed_in.clone()).and_modify(|c| *c += 1).or_insert(1);
                }
            }
        }
    }

    for movies in genre_movies.values() {
        let movies_vec: Vec<&String> = movies.iter().collect();
        for (i, movie1) in movies_vec.iter().enumerate() {
            for movie2 in movies_vec.iter().skip(i + 1) {
                graph.add_edge(movie1, movie2);
            }
        }
    }

    let vertices = graph.get_vertices();
    if let Some(start_vertex) = vertices.first().cloned() {
        if let Some(end_vertex) = vertices.last().cloned() {
            if let Some(distance) = graph.calculate_distance(&start_vertex, &end_vertex) {
                println!("Distance between {} and {} is: {}", start_vertex, end_vertex, distance);
            } else {
                println!("No path exists between {} and {}.", start_vertex, end_vertex);
            }
        } else {
            println!("Graph is empty or contains only one vertex.");
        }
    } else {
        println!("Graph is empty.");
    }

    let mut horror_movie_ratios: HashMap<String, f64> = HashMap::new();
    for (genre, horror_count) in &horror_movie_counts {
        if let Some(total_count) = total_movie_counts.get(genre) {
            let ratio = *horror_count as f64 / *total_count as f64;
            horror_movie_ratios.insert(genre.clone(), ratio);
        }
    }

    let mut sorted_ratios: Vec<_> = horror_movie_ratios.iter().collect();
    sorted_ratios.sort_by(|(_, &a), (_, &b)| b.partial_cmp(&a).unwrap());

    let categories: Vec<&str> = sorted_ratios.iter().map(|(genre, _)| genre.as_str()).collect();
    let data: Vec<(usize, f64)> = sorted_ratios.iter().enumerate().map(|(i, (_, &ratio))| (i, ratio)).collect();

    let max_ratio = data.iter().map(|(_, ratio)| *ratio).fold(f64::NEG_INFINITY, f64::max);
    let root = BitMapBackend::new("Horror_Movies_by_Genres.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Ratio of Horror Movies by Genre", ("Arial", 20).into_font())
        .build_cartesian_2d(
            0usize..categories.len(),
            0.0..max_ratio,
        )?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(ShapeStyle::from(&BLUE).filled())
            .data(data.iter().map(|(index, ratio)| (*index, *ratio))),
    )?;

    Ok(())
}