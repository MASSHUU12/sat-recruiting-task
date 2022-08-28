# SAT Recruitment Task

This API is a recruitment task commissioned by Software Angels Technologies.

The compiled version for `Windows` is available [here](https://github.com/MASSHUU12/sat-recruiting-task/releases).

- [SAT Recruitment Task](#sat-recruitment-task)
- [Development setup](#development-setup)
  - [Running in Docker](#running-in-docker)
  - [Fix for possible error when using with VSCode:](#fix-for-possible-error-when-using-with-vscode)
- [API calls](#api-calls)
  - [GET: /calculateDieselUsageForDistance](#get-calculatedieselusagefordistance)
    - [Implementation notes](#implementation-notes)
    - [Parameters](#parameters)
    - [Response messages](#response-messages)
  - [GET: /probabilityOfUnitInjectorFail](#get-probabilityofunitinjectorfail)
    - [Implementation notes](#implementation-notes-1)
    - [Parameters](#parameters-1)
    - [Response messages](#response-messages-1)

# Development setup

> Make sure you have Rust installed.

Copy the `.env.example` file, rename it to `.env`, and run:

> In the .env file you can customize the port and IP address on which the server will run.

```sh
> cargo run
```

If you want to build release ready app, use:

```sh
> cargo build --release
```

Env file is only useful if you don't use Docker, steps for running program in Docker can be found [below](#running-in-docker).

## Running in Docker

> Make sure that Docker is running in the background.

> For some reason the host does not have access to the container, it is possible that this is a bug only with me, so I leave this section in the documentation.

To create a new container use (this may take a while, because Docker has to download all the necessary things and compile the program):

```sh
> docker build -t sat-recruiting-task .
```

Then, to start the container:

```sh
> docker run -p 8080:3030 --rm --name sat-task sat-recruiting-task
```

or, to run in detached mode:

```sh
> docker run -dp 8080:3030 --rm --name sat-task sat-recruiting-task
```

Regardless of how you run app, you will get some info in console. Ignore it, in the case of running the program in Docker it does not matter:

```text
Variable PORT is not specified in .env file, using default: 3030.
Variable IP is not specified in .env file, using default: 127.0.0.1
```

## Fix for possible error when using with VSCode:

If you are getting error: `unresolved import 'rand' use of undeclared crate or module 'rand'` or something similar, try adding this to settings.json (VSCode settings).

```json
"code-runner.executorMap": {
   "rust": "cargo run # $fileName"
}
```

# API calls

## GET: /calculateDieselUsageForDistance

### Implementation notes

This endpoint returns fuel usage based on input.

Response Content Type: `application/json`

Authorization token: `none`

### Parameters

| Parameter         | Value    | Description                                  | Parameter type | Data type |
| ----------------- | -------- | -------------------------------------------- | -------------- | --------- |
| distance          | Required | Total distance between points A and B in KM. | Query          | Integer   |
| yearOfProduction  | Required | Year of production of the car.               | Query          | Integer   |
| fuelUsagePer100KM | Required | Approximate fuel consumption per 100KM.      | Query          | Integer   |

### Response messages

<table>

<tr>
    <th>Code</th>
    <th>Reason</th>
    <th>Response</th>
</tr>

<tr>
<td>200</td>
<td>Returns fuel usage.</td>
<td>

```json
{
  "fuelUsage": 6
}
```

</td>
</tr>

<tr>
<td>400</td>
<td>Returns info about user error.</td>
<td>

```json
{
  "err": [
    "Parameter 'distance' is invalid: 250km",
    "Parameter 'yearOfProduction' is invalid: dwa tysiące sześć",
    "Parameter 'fuelUsagePer100KM' is invalid: 10 na 100 km"
  ]
}
```

</td>
</tr>

</table>

## GET: /probabilityOfUnitInjectorFail

### Implementation notes

This endpoint returns a percentage of the chance that the engine of the C6 model will fail on the Unit Injector element.

Response Content Type: `application/json`

Authorization token: `none`

### Parameters

| Parameter | Value    | Description                                              | Parameter type | Data type |
| --------- | -------- | -------------------------------------------------------- | -------------- | --------- |
| vin       | Required | I dunno what this is, but customer really wants it here. | Query          | String    |

### Response messages

<table>

<tr>
    <th>Code</th>
    <th>Reason</th>
    <th>Response</th>
</tr>

<tr>
<td>200</td>
<td>Returns a percentage of the chance that the engine will fail.</td>
<td>

```json
{
  "failProbability": "0,77"
}
```

</td>
</tr>

<tr>
<td>400</td>
<td>Returns info about user error.</td>
<td>

```json
{
  "err": "Parameter vin is invalid: 2t5i1011111111111d"
}
```

</td>
</tr>

</table>
