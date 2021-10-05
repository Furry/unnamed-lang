// Area is equal to Pi * R^2
function areaOfCircle(radius) {
    return Math.PI * radius^2;   
}

function checkEvenOdd() {
    for (x = 0; x < 6; x++) {
        if (x % 2 == 0) {
            console.log(x + " is even");
        } else {
            console.log(x + " is odd");
        }
    }
}

function max(num1, num2) {
    if (num1 > num2) {
        return num1;
    } else {
        return num2;
    }
}

function checkTemp(num) {
    if (num < 68) {
        return console.log("Too Cold for Florida");
    }

    if (num < 84) {
        return console.log("Pleasant");
    }

    return console.log("Sunny");
}

function maxAlert(x, y) {
    window.prompt(max(x, y));
}

function fizzBuzz() {
    for (x = 0; x < 100; x++) {
        let res = "";
        if (x % 3 == 0) {
            res += "Fizz";
        }
        if (x % 5 == 0) {
            res += "Buzz";
        }

        if (res.length == 0) {
            console.log(x);
        } else {
            console.log(res);
        }
    }
}

function time() {
    const days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    const date = Date.now();
    const dateObj = new Date(date);

    console.log(`Today is : ${days[dateObj.getDay()]}\nThe current time is ${dateObj.getHours()} : ${dateObj.getMinutes()} : ${dateObj.getSeconds()}`);
}