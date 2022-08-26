# SAT Recruitment Task

These API are a recruitment task commissioned by Software Angels Technologies.

# Technologies used

- Rust
- Docker?
- ...

# Development setup

## Dependencies documentations:

- [Warp](https://crates.io/crates/warp)

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
