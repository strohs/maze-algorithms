package com.mazes;

/**
 * utility functions used by the different maze generation algorithms
 */
public class Util {
    /**
     * @return a random integer between min (inclusive) and max (exclusive)
     */
    public static int randRange(int min, int max) {
        return (int) (Math.random() * (max - min)) + min;
    }

    /**
     * @return a random integer between 0 and max (exclusive)
     */
    public static int rand(int max) {
        return (int) (Math.random() * max);
    }

    /**
     * shuffles the elements of the passed in array in place
     * @param arr - array of elements to shuffle
     * @param <T> - the type of array element
     */
    public static<T> void shuffle(T [] arr) {
        for (int i = arr.length - 1; i > 0; i--) {
            var rand = rand(i+1);
            var temp = arr[rand];
            arr[rand] = arr[i];
            arr[i] = temp;
        }
    }
}
