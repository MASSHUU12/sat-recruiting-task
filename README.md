# SAT Recruitment Task

This API is a recruitment task commissioned by Software Angels Technologies.

The compiled version for `Windows` is available [here](#).

- [SAT Recruitment Task](#sat-recruitment-task)
- [Technologies used](#technologies-used)
- [Development setup](#development-setup)
  - [Fix for possible error when using with VSCode:](#fix-for-possible-error-when-using-with-vscode)
- [API calls](#api-calls)
  - [GET: /calculateDieselUsageForDistance?distance=?yearOfProduction=?fuelUsagePer100KM=](#get-calculatedieselusagefordistancedistanceyearofproductionfuelusageper100km)
    - [Implementation notes:](#implementation-notes)
    - [Parameters](#parameters)
    - [Response messages](#response-messages)
  - [GET: /probabilityOfUnitInjectorFail](#get-probabilityofunitinjectorfail)
    - [Implementation notes:](#implementation-notes-1)
    - [Parameters](#parameters-1)
    - [Response messages](#response-messages-1)

# Technologies used

- Rust ^1.63.0

# Development setup

> Make sure you have Rust installed.

Copy the `.env.example` file and rename it to `.env`, and run:

```sh
> cargo run
```

## Fix for possible error when using with VSCode:

If you are getting error: `unresolved import 'rand' use of undeclared crate or module 'rand'`, try adding this to settings.json.

```json
"code-runner.executorMap": {
   "rust": "cargo run # $fileName"
}
```

# API calls

## GET: /calculateDieselUsageForDistance?distance=?yearOfProduction=?fuelUsagePer100KM=

### Implementation notes:

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
    <th>Parameter</th>
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

</table>

## GET: /probabilityOfUnitInjectorFail

### Implementation notes:

This endpoint returns a percentage of the chance that the engine of the C6 model will fail on the Unit Injector element.

Response Content Type: `application/json`

Authorization token: `none`

### Parameters

| Parameter | Value    | Description                                              | Parameter type | Data type |
| --------- | -------- | -------------------------------------------------------- | -------------- | --------- |
| vin       | Required | I dunno what this is, but customer really wants it here. | Query          | String?   |

### Response messages

<table>

<tr>
    <th>Parameter</th>
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

</table>
