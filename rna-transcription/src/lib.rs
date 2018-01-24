use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: Vec<char>
}

#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: Vec<char>
}

impl RNA {
    pub fn new(string: &str) -> Self {
        let nucleotides = string_to_nucleotides(string);

        RNA { nucleotides }
    }

    pub fn to_dna(&self) -> Result<DNA, &'static str> {
        let hash: HashMap<char, char> = [
            ('C','G'), ('G','C'), ('A','T'), ('U','A')
        ].iter().cloned().collect();

        let nucleotides = self.nucleotides.iter().map(|ch| {
            match hash.get(&ch) {
                None      => 'Z',
                Some(val) => *val
            }
        }).collect::<Vec<char>>();

        if nucleotides.iter().any(|ch| *ch == 'Z') { return Err("oops") }

        Ok(DNA{ nucleotides })
    }
}

impl DNA {
    pub fn new(string: &str) -> Self {
        let nucleotides = string_to_nucleotides(string);

        DNA { nucleotides }
    }


    pub fn to_rna(&self) -> Result<RNA, &'static str>  {
        let hash: HashMap<char, char> = [
            ('G','C'), ('C','G'), ('T','A'), ('A','U')
        ].iter().cloned().collect();

        let nucleotides = self.nucleotides.iter().map(|ch| {
            match hash.get(&ch) {
                None      => 'Z',
                Some(val) => *val
            }
        }).collect::<Vec<char>>();

        if nucleotides.iter().any(|ch| *ch == 'Z') { return Err("oops") }

        Ok(RNA{ nucleotides })
    }
}

fn string_to_nucleotides(string: &str) -> Vec<char> {
    string.chars().collect::<Vec<char>>()
}
