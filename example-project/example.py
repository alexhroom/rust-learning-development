import numpy as np

from example_project import list_sum_string, array_sum_string

print(list_sum_string([1, 2, 3, 4, 5]))

# note we have to tell numpy to use Fortran data ordering as that's what Rust uses... 
print(array_sum_string(np.array([[1., 2., 3.], [4., 5., 6.]], order='F')))
