//set input to var

//run in box when button el is pressed
document.getElementById('button').addEventListener('click', function() {
    var input = document.getElementById('input').value;
    if (input == null) {
        alert('Please enter a valid input');
    }
    var output = document.getElementById('output');
    output.innerHTML = eval(input);
});
