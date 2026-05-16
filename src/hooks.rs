//! Custom hooks and utilities for the PaoValues application

use dioxus::prelude::*;
use std::collections::HashMap;

/// Hook for managing component selection state
#[derive(Clone, Debug)]
pub struct ComponentSelection {
    pub selected_logic: Signal<Option<usize>>,
    pub selected_visual: Signal<Option<usize>>,
    pub hovered_component: Signal<Option<(String, usize)>>, // (type, id)
}

/// Create a component selection hook
pub fn use_component_selection() -> ComponentSelection {
    let selected_logic = use_signal(|| None);
    let selected_visual = use_signal(|| None);
    let hovered_component = use_signal(|| None);

    ComponentSelection {
        selected_logic,
        selected_visual,
        hovered_component,
    }
}

/// Hook for managing search and filtering state
#[derive(Clone, Debug)]
pub struct SearchState {
    pub search_query: Signal<String>,
    pub selected_category: Signal<String>,
    pub show_explanations: Signal<bool>,
    pub sort_by: Signal<String>, // "name", "category", "id"
    pub sort_direction: Signal<String>, // "asc", "desc"
}

/// Create a search state hook
pub fn use_search_state() -> SearchState {
    let search_query = use_signal(|| String::new());
    let selected_category = use_signal(|| String::from("All"));
    let show_explanations = use_signal(|| true);
    let sort_by = use_signal(|| String::from("name"));
    let sort_direction = use_signal(|| String::from("asc"));

    SearchState {
        search_query,
        selected_category,
        show_explanations,
        sort_by,
        sort_direction,
    }
}

/// Hook for managing PAO navigation state
#[derive(Clone, Debug)]
pub struct PAONavigation {
    pub selected_pao_index: Signal<Option<usize>>,
    pub pao_history: Signal<Vec<usize>>,
    pub favorite_indices: Signal<Vec<usize>>,
}

/// Create a PAO navigation hook
pub fn use_pao_navigation() -> PAONavigation {
    let selected_pao_index = use_signal(|| None);
    let pao_history = use_signal(|| Vec::new());
    let favorite_indices = use_signal(|| Vec::new());

    PAONavigation {
        selected_pao_index,
        pao_history,
        favorite_indices,
    }
}

/// Hook for managing component relationships
#[derive(Clone, Debug)]
pub struct ComponentRelationships {
    pub related_components: Signal<HashMap<usize, Vec<usize>>>, // Logic ID -> Visual IDs
    pub dependency_map: Signal<HashMap<usize, Vec<usize>>>, // Component ID -> Dependency IDs
}

/// Create a component relationships hook
pub fn use_component_relationships() -> ComponentRelationships {
    let related_components = use_signal(|| HashMap::new());
    let dependency_map = use_signal(|| HashMap::new());

    ComponentRelationships {
        related_components,
        dependency_map,
    }
}

/// Utility function to get related components
pub fn get_related_components(logic_id: usize) -> Vec<usize> {
    // For now, we'll use a simple mapping where Logic ID N maps to Visual ID N
    // In a real application, this could be more sophisticated
    vec![logic_id]
}

/// Utility function to get component dependencies
pub fn get_component_dependencies(component_id: usize, component_type: &str) -> Vec<usize> {
    // Simple dependency rules for demonstration
    let mut dependencies = Vec::new();
    
    match component_type {
        "logic" => {
            // Logic components might depend on lower-numbered logic components
            if component_id > 1 {
                dependencies.push(component_id - 1);
            }
        },
        "visual" => {
            // Visual elements might depend on lower-numbered visual elements
            if component_id > 1 {
                dependencies.push(component_id - 1);
            }
        },
        _ => {}
    }
    
    dependencies
}

/// Utility function to filter components by search criteria
pub fn filter_components(
    components: Vec<crate::data::CodingComponent>,
    search_query: &str,
    selected_category: &str,
) -> Vec<crate::data::CodingComponent> {
    let query = search_query.to_lowercase();
    
    components
        .into_iter()
        .filter(|c| {
            let matches_search = query.is_empty()
                || c.name.to_lowercase().contains(&query)
                || c.category.to_lowercase().contains(&query)
                || c.description.to_lowercase().contains(&query);
            let matches_category = selected_category == "All"
                || selected_category == c.category;
            matches_search && matches_category
        })
        .collect()
}

/// Utility function to sort components
pub fn sort_components(
    mut components: Vec<crate::data::CodingComponent>,
    sort_by: &str,
    sort_direction: &str,
) -> Vec<crate::data::CodingComponent> {
    let ascending = sort_direction == "asc";
    
    match sort_by {
        "name" => {
            if ascending {
                components.sort_by(|a, b| a.name.cmp(&b.name));
            } else {
                components.sort_by(|a, b| b.name.cmp(&a.name));
            }
        },
        "category" => {
            if ascending {
                components.sort_by(|a, b| a.category.cmp(&b.category));
            } else {
                components.sort_by(|a, b| b.category.cmp(&a.category));
            }
        },
        "id" => {
            if ascending {
                components.sort_by(|a, b| a.id.cmp(&b.id));
            } else {
                components.sort_by(|a, b| b.id.cmp(&a.id));
            }
        },
        _ => {}
    }
    
    components
}

/// Hook for local storage management (simplified for both platforms)
pub fn use_local_storage<T: serde::Serialize + serde::de::DeserializeOwned + Clone + Default + 'static>(
    key: &'static str,
    default: T,
) -> (Signal<T>, impl FnMut(T) + 'static) {
    let mut value = use_signal(|| {
        // Try to load from localStorage (web only)
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(stored)) = storage.get_item(key) {
                        if let Ok(parsed) = serde_json::from_str::<T>(&stored) {
                            return parsed;
                        }
                    }
                }
            }
        }
        default
    });

    let save = move |new_value: T| {
        value.set(new_value.clone());
        
        // Save to localStorage (web only)
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(json) = serde_json::to_string(&new_value) {
                        let _ = storage.set_item(key, &json);
                    }
                }
            }
        }
    };

    (value, save)
}

/// Memory aid generator for PAO components
pub fn generate_memory_aid(pao_index: &crate::data::PAOIndex) -> String {
    format!(
        "Remember: {} / {} / {} → Logic: {} + Visual: {}",
        pao_index.person,
        pao_index.action,
        pao_index.object,
        pao_index.logic_component.name,
        pao_index.visual_element.name
    )
}

/// Hook for generating memory aids
pub fn use_memory_aids() -> impl Fn(usize) -> String {
    let pao_system = crate::data::get_pao_system();
    
    move |index| {
        if let Some(pao) = pao_system.get(index) {
            generate_memory_aid(pao)
        } else {
            "Memory aid not available".to_string()
        }
    }
}

/// Performance metrics hook
#[derive(Clone, Debug)]
pub struct PerformanceMetrics {
    pub render_count: Signal<usize>,
    pub search_time: Signal<f64>,
    pub component_count: Signal<usize>,
}

/// Create a performance metrics hook
pub fn use_performance_metrics() -> PerformanceMetrics {
    let mut render_count = use_signal(|| 0);
    let search_time = use_signal(|| 0.0);
    let component_count = use_signal(|| 0);

    // Increment render count
    let current = *render_count.read();
    render_count.set(current + 1);

    PerformanceMetrics {
        render_count,
        search_time,
        component_count,
    }
}
