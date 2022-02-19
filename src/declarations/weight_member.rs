use rand::Rng;

pub struct WeightMember {
    pub weight: f64,
    name: String,
}

impl WeightMember {
    pub fn get_weight_list_from_vec(vec: &Vec<String>) -> Vec<Self> {
        vec.iter()
            .map(|name| {
                let weight: f64 = rand::thread_rng().gen();

                WeightMember {
                    name: name.clone(),
                    weight,
                }
            })
            .collect()
    }

    pub fn get_vec_from_weight_list(vec: &Vec<WeightMember>) -> Vec<String> {
        vec.iter()
            .map(|weight_member| weight_member.name.clone())
            .collect()
    }
}
