/**
 * returns a random integer from 0 to max (exclusive)
 * @param max
 * @returns {number}
 */
function randomInt(max) {
  return Math.floor(Math.random() * Math.floor(max));
}

/**
 * shuffles the elements of the passed in array, in place, using Knuth shuffle
 * @param array - the array to shuffle
 * @returns {*} the passed in array
 */
function shuffle(array) {
  let curIndex = array.length;
  let tempValue, randIndex;

  // work from back to front of the array
  while (0 !== curIndex) {
    randIndex = randomInt(curIndex);
    curIndex -= 1;

    // swap array[curIndex] and array[randIndex]
    tempValue = array[curIndex];
    array[curIndex] = array[randIndex];
    array[randIndex] = tempValue;
  }

  return array;
}

/**
 * returns a random boolean
 * @returns {boolean}
 */
function randomBoolean() {
  return Math.random() > 0.5;
}

/**
 * returns (but does not remove) a random element from the passed in Array
 * @param array
 * @returns {*}
 */
function sample(array) {
  return array[randomInt(array.length)];
}

exports.randomInt = randomInt;
exports.shuffle = shuffle;
exports.randomBoolean = randomBoolean;
exports.sample = sample;