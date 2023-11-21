#include <cstdio>

void print_sorting_step(int array[], int size, bool skip_visualization);
void print_all(int arr[], int size);

//Diese Funktion fügt den Wert an der Stelle i an der richtigen Stelle im
//    sortierten Berech array[0,i-1] ein
void insert(int array[], int i){
  /*
    Ergänzen Sie hier den Code
  */
  printf("%i\n", i);
}

void insertion_sort(int array[], int size) {
  /*
    Ergänzen Sie hier den Code
  */
  for(int i = 0; i < size; ++i) {
    insert(array, i);
  }
}

int main() {
  char filename[1000];
  printf("Enter a filename that contains the given array:\n");
  scanf("%s", filename);

  FILE* file = fopen(filename, "r");
  rewind(file);
  int size;
  fscanf(file, "%i", &size);
  int* arr = new int[size];
  for (int i = 0; i < size; ++i) {
    fscanf(file, "%i", &arr[i]);
  }
  fclose(file);

  printf("The size of the array is %i.\n", size);
  printf("The array contains the following values:\n");
  print_all(arr, size);
  
  insertion_sort(arr, size);

  delete[] arr;

  return 0;
}

void print_all(int arr[], int size){
  printf("[%i", arr[0]);
  for (int i = 1; i < size; ++i) {
    printf(", %i", arr[i]);
  }
  printf("]\n");
}

void print_sorting_step(int array[], int size, bool skip_visualization){
  printf("===== Printing sorting step =====\n");
  print_all(array, size);
  
  if(skip_visualization)
    return;
  
  for(int v = 0; v < size; v++){
    printf("%i : ", v);
    for(int s = 0; s < array[v]; s++)
      printf("#");
    printf("\n");
  }
}
