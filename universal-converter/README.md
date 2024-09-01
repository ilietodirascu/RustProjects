# Custom Universal Converter

## Introduction

In this project, we will be creating a universal converter. The converter will be able to convert various units of measurement, such as length, temperature, weight and currency, from one system to another.

## Technologies Used

- Programming Language: RUST

## Installation

1. Clone the repository: `git clone [repository URL]`
2. Navigate to the project directory: `cd universal-converter`
3. Install the required dependencies: `npm install`

## Usage

1. Open the terminal and navigate to the project directory.
2. Run the converter: `npm start`
3. Follow the on-screen instructions to select the units and enter the values you want to convert.

## Features

- Conversion of length units (e.g., meters to feet, kilometers to miles)
- Conversion of temperature units (e.g., Celsius to Fahrenheit, Kelvin to Celsius)
- Conversion of weight units (e.g., kilograms to pounds, ounces to grams)

## Contributing

1. Fork the repository.
2. Create a new branch: `git checkout -b feature/your-feature-name`
3. Make your changes and commit them: `git commit -m 'Add your feature'`
4. Push to the branch: `git push origin feature/your-feature-name`
5. Submit a pull request.

## License

This project is licensed under the [Insert license here]. See the [LICENSE](LICENSE) file for more details.

## Contact

For any questions or suggestions, please feel free to reach out to us at [insert contact email here].

## Temp Details

We create a console application that converts units. Units can be converted as long as they measure the same thing. Therefore we need types. Such as Distance, Weight, Power, etc.
`convert -t Distance -u m km 100` we would get 0.1 km.
To simplify things we should introduce the option to ommit parameters.
The lowest level of specificity would be
`convert m km 100`.

This implies an implementation where the type of the dimension we are measuring is determined based on the first argument m. The program needs to loop over all unit abbreviations of each category and find the first category that contains m.

If such category is found the program does an additional check within the found category if it contains the km shortcut.

Cases where this wouldn't work. If there are collisions of units. Maybe the intended category is Physics but the program picked a different Category that contains the first argument. In case by chance the second argument is also contained in the unintended category the program will run incorrectly.
Therefore we also need to do an additional check the first time we search for the first argument, to make sure only one category contains it.

Categories aren't allowed to contain 2 or more units with the same name. Thus only the first argument needs additional checks.

If there are collisions the program needs to return appropriate errors.

The collisions can be circumvented by increasing specificity.

`convert m km 100` becomes ` convert -t Physics m km 100`

The type doesn't have to be case sensitive physics = Physics, whereas the units have to. G is different from g.

### How do Converters work

We need a generic Converter trait that has the following methods

`convert(f64 value, Unit from, Unit To)`

So if we know the type of the conversion is Distance we instantiate a DistanceConverter.

If the arguments are dm km. The converter will invoke the convert method and pass the value to be coverted and the Units. Then the first unit will invoke his `to_gold_standard(f64 value)` method. So our dm Unit will convert 100 to 10 meters. Then the second unit km will invoke his `from_gold_standard(10)` method and return 0.01 km.

The Unit trait has 2 methods from_gold_standard(T value) and to_gold_standard(T value)

Each converter knows the gold standard for his units. Each converter has a list of supported Units.

Each unit knows only how to convert to and from the gold standard.

### Parsing

The minimal amount of parameters can be seen in this command:
`convert m km 100` where only the units and the value to convert are provided.

To achieve this we need to check if we have been given the Conversion Type.

Case 1 We have been given the conversion type with this command.

`convert -t Distance m km 100 `

a) Check if the parsed value is contained in the ConversionTypes union enum. If not return error message.

b) The type is valid, check if the units are valid. Each conversion type such as Distance has it's own hashmap. The key is the unit abbreviation string such as km, and the value is the appropriate enum with the configured multiplier.

If the type is provided, the hashmap of the appropriate type is queried. If not a vec of all these hashmaps is iterated and if there is only one hashmap whose key list contains both the first and second argument the conversion is made.
If not ambiguity error is thrown and the user is prompted to increase the specificity.

### Currency

This logic applies for when the user is attempting to convert currencies only.

1. Check if currencies.json exists, is not empty, and is not older than 24h
2. If any statement from 1. is false make an api call to update/get the currencies and update the file.
3. Implement a "static" function that would process the json file and return a hashmap of measurements and their respective keys.
