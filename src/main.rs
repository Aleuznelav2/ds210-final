extern crate csv;
use serde::Deserialize;
use std::collections::VecDeque;
#[derive(Debug, Deserialize)]

struct Subreddits {
    source_subreddit: Vec<String>,
    target_subreddit: Vec<String>,
}

impl Subreddits{
    fn new() -> Subreddits{
        Subreddits{
            source_subreddit: Vec::new(),
            target_subreddit: Vec::new(),
        }
    }

    fn read_csv(filepath: &str, has_headers: bool) -> Subreddits{
        let file = std::fs::File::open(filepath).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);


        let mut data_frame = Subreddits::new();
    
        for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
            
            //println!("{:?}", record);
        }
    
        return data_frame;
    }

    fn push(&mut self, row: &csv::StringRecord){
        self.source_subreddit.push(row[0].to_string());
        self.target_subreddit.push(row[1].parse().unwrap());
    }
}

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

// reverse direction of edges on a list
fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn create_undirected(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
}

fn main(){
    //use std::time::Instant;
    //let now = Instant::now();

    //let data = Subreddits::read_csv("./reddit_dataset.csv",true);             // I don't want to imagine how long it would take this to compile
    //let data = Subreddits::read_csv("./reddit_dataset_10k.csv",true);         // this took 7 minutes to compile
    let data = Subreddits::read_csv("./reddit_dataset_cond.csv",true);          // condensed set for testing purposes

    //convert struct columns to vector
    let mut subreddit_vector = vec![];

    for j in 0..data.source_subreddit.len(){
       
            subreddit_vector.push((&data.source_subreddit[j],&data.target_subreddit[j]));

        //println!("{:?}", data.source_subreddit[j]);

    }

    //println!("{:?}", subreddit_vector);

    // give each subreddit a specific index. if subreddit repeats, get its old index
    let mut source_index = vec![];
    let mut target_index = vec![];
    let mut overall_index = 0;

    // fixes indexing issues
    let mut eggwoll = 0;        // don't push a subreddit again with a new index if already existed before
    let mut vert = 0;           // if a subreddit appears multiple times, subtract its occurences from the new index

    for k in 0..data.source_subreddit.len(){
        for m in 0..k {
            if source_index.contains(&(&data.source_subreddit[k].to_owned(),m)) { 
                source_index.push((&data.source_subreddit[k],m));
                eggwoll = 1;
                vert += 1;
                break;
            }
        }

        if eggwoll == 0{
            source_index.push((&data.source_subreddit[k],k-vert));
            overall_index += 1;
        }

        eggwoll = 0;
    }
        
    //println!("{:?} \n ", source_index);     // source subreddits indexed

    let mut hayashi = 0;    // these are the same as eggwoll and vert, just for target subreddits
    let mut seeds = 0;
    let mut frost = 0;      // if target was already in source, don't recreate target from target list

    for k in 0..data.target_subreddit.len(){
        // if target subreddit is already in source list
        for m in 0..overall_index{
            if source_index.contains(&(&data.target_subreddit[k].to_owned(),m)) { 
                target_index.push((&data.target_subreddit[k],m));
                hayashi += 1;
                seeds += 1;
                frost += 1;
                break;
            }
        }

        // if target subreddit is already in target list

        if frost == 0{
            for m in 0..overall_index + k {
                if target_index.contains(&(&data.target_subreddit[k].to_owned(),m)) { 
                    target_index.push((&data.target_subreddit[k],m));
                    hayashi += 1;
                    seeds += 1;
                    break;
                }
            }
        }

        if hayashi == 0{
            target_index.push((&data.target_subreddit[k],k-seeds+overall_index));
        }
        
        hayashi = 0;
        frost = 0;
    }

    //println!("{:?} \n ", target_index);     // target subreddits indexed

    // master subreddit list with their indexes instead of name 
    let mut subreddit_indexed = vec![];

    for k in 0..data.source_subreddit.len(){
        subreddit_indexed.push((source_index[k].1,target_index[k].1));
    }
    
    // complete subreddit list to grab their names
    let mut subreddit_list = vec![];

    eggwoll = 0;
    vert = 0;

    for k in 0..data.source_subreddit.len(){
        for m in 0..k {
            if subreddit_list.contains(&(&data.source_subreddit[k].to_owned(),m)) { 
                //source_index.push((&data.source_subreddit[k],m));
                eggwoll = 1;
                vert += 1;
                break;
            }
        }

        if eggwoll == 0{
            subreddit_list.push((&data.source_subreddit[k],k-vert));
            //overall_index += 1;
        }

        eggwoll = 0;
    }

    hayashi = 0;    
    seeds = 0;
    frost = 0;      

    for k in 0..data.target_subreddit.len(){
        // if target subreddit is already in source list
        for m in 0..overall_index{
            if subreddit_list.contains(&(&data.target_subreddit[k].to_owned(),m)) { 
                //target_index.push((&data.target_subreddit[k],m));
                hayashi += 1;
                seeds += 1;
                frost += 1;
                break;
            }
        }

        // if target subreddit is already in target list

        if frost == 0{
            for m in 0..overall_index + k {
                if subreddit_list.contains(&(&data.target_subreddit[k].to_owned(),m)) { 
                    //target_index.push((&data.target_subreddit[k],m));
                    hayashi += 1;
                    seeds += 1;
                    break;
                }
            }
        }

        if hayashi == 0{
            subreddit_list.push((&data.target_subreddit[k],k-seeds+overall_index));
        }
        
        hayashi = 0;
        frost = 0;
    }

    println!("{:?}", subreddit_list);

    
    //println!("{:?}", subreddit_indexed);

    //let elapsed = now.elapsed();
    //println!("Elapsed: {:.2?}", elapsed);

    let n: usize = 368;
    let mut edges: ListOfEdges = subreddit_indexed;
    edges.sort();
    println!("{:?}",edges);
    let graph = Graph::create_undirected(n,&edges);
    for (i, l) in graph.outedges.iter().enumerate() {
        println!("{} {:?}", i, *l);
    }

    let start: Vertex = 113; // start at league of legends

    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0); // <= we know this distance
    println!("{:?}", distance);

    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    println!("{:?}", queue);

    println!("{:?}",queue);

    while let Some(v) = queue.pop_front() { // new unprocessed vertex
        println!("top {:?}",queue);
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { // consider all unprocessed neighbors of v
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
                println!("In {:?}",queue);
            }
        }
    };
    
    print!("vertex:distance");
    for v in 0..graph.n {
        print!("    {}:{}",v,distance[v].unwrap());
    }
    println!();
}
