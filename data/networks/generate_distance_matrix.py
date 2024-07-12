import numpy as np
import argparse
import os

def generate_random_distance_matrix(n, min_distance=1.0, max_distance=100.0):
    """
    Generates a random distance matrix of size n x n.

    Parameters:
    n (int): The number of points.
    min_distance (int): The minimum possible distance between points.
    max_distance (int): The maximum possible distance between points.

    Returns:
    numpy.ndarray: A n x n distance matrix with non-negative values and zeros on the diagonal.
    """
    if min_distance > max_distance:
        raise ValueError("min_distance cannot be greater than max_distance")
        
    # Create a random matrix with values between min_distance and max_distance
    matrix = np.random.uniform(min_distance, max_distance, size=(n, n))
    
    # Set the diagonal to 0, as the distance from a point to itself is 0
    np.fill_diagonal(matrix, 0)

    return matrix

def save_matrix_to_txt(matrix, filename, n):
    """
    Saves a given matrix to a text file.

    Parameters:
    matrix (numpy.ndarray): The matrix to save.
    filename (str): The name of the file to save the matrix in.
    n (int): The size of the matrix to include in the file.
    """
    with open(filename, 'w') as f:
        f.write(f"{n}\n")
        np.savetxt(f, matrix, fmt="%.2f")

def main():
    parser = argparse.ArgumentParser(description="Generate a random distance matrix and save it to a file.")
    parser.add_argument('n', type=int, help="Size of the matrix (number of points)")
    parser.add_argument('min_distance', type=float, help="Minimum possible distance between points")
    parser.add_argument('max_distance', type=float, help="Maximum possible distance between points")

    args = parser.parse_args()

    # Create the directory
    directory_name = f"random_uniform_{args.n}_{int(args.min_distance)}_{int(args.max_distance)}"
    os.makedirs(directory_name, exist_ok=True)

    # Full path for the file
    file_path = os.path.join(directory_name, "distance_matrix.txt")

    distance_matrix = generate_random_distance_matrix(args.n, args.min_distance, args.max_distance)
    print("Random Distance Matrix:")
    print(distance_matrix)

    save_matrix_to_txt(distance_matrix, file_path, args.n)
    print(f"Matrix saved to {file_path}")

if __name__ == "__main__":
    main()
