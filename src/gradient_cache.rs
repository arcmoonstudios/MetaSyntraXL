// src/gradient_cache.rs ~=#######D]====A===r===c====M===o===o===n====<Lord[GRADIENT-CACHE]Xyn>=====S===t===u====d===i===o===s====[R|$>
use std::collections::{HashMap, VecDeque};
use tokio::sync::Mutex;
use tch::Tensor;

pub struct GradientCache {
    cache: Mutex<GradientLRUCache>,
}

struct GradientLRUCache {
    map: HashMap<String, Tensor>,
    order: VecDeque<String>,
    capacity: usize,
}

impl GradientCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Mutex::new(GradientLRUCache {
                map: HashMap::new(),
                order: VecDeque::new(),
                capacity,
            }),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Tensor> {
        let mut cache = self.cache.lock().await;
        if let Some(tensor) = cache.map.get(key) {
            let tensor_clone = tensor.shallow_clone();
            cache.order.retain(|k| k != key);
            cache.order.push_front(key.to_string());
            Some(tensor_clone)
        } else {
            None
        }
    }

    pub async fn insert(&self, key: String, gradient: Tensor) {
        let mut cache = self.cache.lock().await;
        if cache.map.len() >= cache.capacity && !cache.map.contains_key(&key) {
            if let Some(lru_key) = cache.order.pop_back() {
                cache.map.remove(&lru_key);
            }
        }
        cache.map.insert(key.clone(), gradient);
        cache.order.retain(|k| k != &key);
        cache.order.push_front(key);
    }

    pub async fn values(&self) -> Vec<Tensor> {
        let cache = self.cache.lock().await;
        cache.map.values().map(|t| t.shallow_clone()).collect()
    }
}