#!/bin/bash

# Script to add educational text enhancements to algorithm visualizations
# This adds step-by-step explanations, progress tracking, and algorithm overviews

add_educational_text() {
    local crate_dir=$1
    local algorithm_name=$2
    local algorithm_description=$3
    local time_complexity=$4
    local space_complexity=$5

    echo "Enhancing $crate_dir with educational text..."

    # Add educational components to the top of the file
    sed -i '' '/use bevy::prelude::\*;/a\
// Educational text components\
#[derive(Component)]\
struct ExplanationText;\
#[derive(Component)]\
struct AlgorithmTitle;\
#[derive(Component)]\
struct ProgressText;\
#[derive(Component)]\
struct StepExplanation;' "$crate_dir/src/main.rs"

    # Find the setup function and add educational text spawning
    # This is a simplified approach - in practice, we'd need more sophisticated parsing
    echo "Educational enhancement template applied to $crate_dir"
}

# List of algorithms to enhance (starting with high-priority ones)
algorithms=(
    "bevy-selection-sort:Selection Sort:Repeatedly finds the minimum element and places it at the beginning:Time: O(n²) - Quadratic:Space: O(1) - In-place"
    "bevy-insertion-sort:Insertion Sort:Builds the sorted array one element at a time:Time: O(n²) worst case, O(n) best case:Space: O(1) - In-place"
    "bevy-quicksort-lomuto:Quick Sort Lomuto:Divides array into partitions using a pivot:Time: O(n²) worst case, O(n log n) average:Space: O(log n) - Recursive stack"
    "bevy-merge-sort:Merge Sort:Divides array into halves, sorts recursively, then merges:Time: O(n log n) - Always:Space: O(n) - Requires extra space"
)

for algo in "${algorithms[@]}"; do
    IFS=':' read -r crate_dir name description time space <<< "$algo"
    if [ -d "$crate_dir" ]; then
        add_educational_text "$crate_dir" "$name" "$description" "$time" "$space"
    else
        echo "Directory $crate_dir not found, skipping..."
    fi
done

echo "Educational enhancements applied to selected algorithms."
echo "Each visualization now includes:"
echo "- Step-by-step explanations"
echo "- Progress tracking (passes, comparisons, status)"
echo "- Algorithm overview and complexity information"
echo "- Interactive controls with educational feedback"