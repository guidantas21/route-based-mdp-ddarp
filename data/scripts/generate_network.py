import numpy as np
import argparse
import os

def generate_random_distance_matrix(n, min_distance=1.0, max_distance=100.0):
    if min_distance > max_distance:
        raise ValueError("min_distance cannot be greater than max_distance")
        
    matrix = np.random.uniform(min_distance, max_distance, size=(n, n))
    np.fill_diagonal(matrix, 0)

    return matrix


def generate_revealed_travel_times(approx_travel_time_matrix, min_noise, max_noise):
    if min_noise > max_noise:
        raise ValueError("min_noise cannor be greater than max_noise")

    noise_matrix = np.random.uniform(min_noise, max_noise, size=approx_matrix.shape)
    revealed_travel_time_matrix = approx_matrix * noise_matrix
    
    return revealed_travel_time_matrix

    
# PARSE

parser = argparse.ArgumentParser(
        description="Generate a random distance matrix and save it to a file.")
    
parser.add_argument(
        'n', type=int, help="Size of the matrix (number of points)")
parser.add_argument(
        'min_distance', type=int, help="Minimum possible distance between points")    
parser.add_argument(
        'max_distance', type=int, help="Maximum possible distance between points")
    
args = parser.parse_args()
    

# GENERATE DISTANCE MATRIX

directory_name = f"random_uniform_{args.n}_{int(args.min_distance)}_{int(args.max_distance)}"
os.makedirs(directory_name, exist_ok=True)
distance_file_path = os.path.join(directory_name, "distance_matrix.txt")

distance_matrix = generate_random_distance_matrix(args.n, args.min_distance, args.max_distance)

print("Random Distance Matrix:")
print(distance_matrix)

np.savetxt(distance_file_path, distance_matrix, fmt='%.2f')

print(f"Matrix saved to {distance_matrix_file_path}") 
