import os
import numpy as np
import argparse

def generate_approx_travel_time_matrix(avg_velocity, distance_matrix):
    approx_travel_time_matrix = distance_matrix / avg_velocity

    return approx_travel_time_matrix


def generate_revealed_travel_time_matrix(approx_travel_time_matrix, min_noise, max_noise):
    noise_matrix = np.random.uniform(min_noise, max_noise, size=approx_travel_time_matrix.shape)
    revealed_travel_time_matrix = approx_travel_time_matrix * noise_matrix
    
    return revealed_travel_time_matrix


# PARSE 

parser = argparse.ArgumentParser(
        description="Generate a random distance matrix and save it to a file.")
   
parser.add_argument(
        'distance_matrix_file', type=str, help="Path of the distance_matrix_file")
parser.add_argument(
        'min_noise', type=float, help="Minimum noise")
parser.add_argument(
        'max_noise', type=float, help="Maximum noise")
parser.add_argument(
        'avg_velocity', type=float, help="Vehicle's average velocity")
    
args = parser.parse_args()


# GENERATE REVEALED TRAVEL TIME MATRIX

revealed_travel_time_matrix_file_path = "revealed_travel_time_matrix.txt"

with open(args.distance_matrix_file, 'r') as f:
    size = int(f.readline().strip())
    distance_matrix = np.loadtxt(f)

approx_travel_time_matrix = generate_approx_travel_time_matrix(args.avg_velocity, distance_matrix)

revealed_travel_time_matrix = generate_revealed_travel_time_matrix(
        approx_travel_time_matrix, args.min_noise, args.max_noise)

print("Revealed travel time matrix:")
print(revealed_travel_time_matrix)

with open(revealed_travel_time_matrix_file_path, 'w') as f:
    f.write(f"{size}\n")  # Write the size as the first line
    np.savetxt(f, revealed_travel_time_matrix, fmt='%.2f')

print(f"Matrix saved to {revealed_travel_time_matrix_file_path}")
