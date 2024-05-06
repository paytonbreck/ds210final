# ds210final
Final Project for CDS DS210 Final

CDS DS 210 Final Project: Netflix Analysis

Overview: This project focuses primarly on analyzing the relationship between movie and televisin show categories, such as genres, and comparing them to different aspects of the titles, including the country they are affiliated with. The project includes creating a graph that represents the data set, as well as implementing tests. Finally, the project includes a visualization of the comparison between how many movies/tv shows a country has uploaded onto Netflix compared to the number of specifically horror features a specific country produces. 

Dataset: 

The dataset used in this project contains information about movies and television shows available on Netflix, including the category they belong to, the country of availability, their age rating, their release date, and a breif description. I found the dataset on Kaggle (https://www.kaggle.com/datasets/shivamb/netflix-shows).


Implementation: The project constructs a graph representation of the dataset. Addtionally, the project focuses on average distances between node parings. In the graph construct, the average_distance function calculates the average distance between all of the node pairings its given while the calculate_distance function uses a BFS algorithm to find the shorest path between two vertices. The graph created models the relationships between movies based on their shared generes, such as "Horror" or "Action & Adventure". The vertices defined here are the induvidual movies, while the edges are the move genres under "listen_in". 

Testing: The tests verify that vertices and edges are added correctly to the graph as well as the calculate_distance function that is used.

Visualization: The project adds a visualization aspect: the ratio of horror movies available in each country using the plotters library. After constructing the graph, the project calculates the ratio of horror movies for each country and sorts them in descending order. Then, it generates a histogram chart showing the distribution of horror movie ratios across different countries. However, it is important to note that some of the data under "country" includes movies that come from multiple countries. In that instance, the ratio is likely to be 1, since the country is unique. 
