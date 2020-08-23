import http from 'k6/http';

export let options = {
  scenarios: {
    warmup: {
      executor: 'constant-arrival-rate',
      rate: 5,
      duration: '2m',
      preAllocatedVUs: 10,
      maxVUs: 20,
    },
    load_100: {
      executor: 'constant-vus',
      startTime: '2m',
      vus: 100,
      duration: '2m',
    },
    load_200: {
      executor: 'constant-vus',
      startTime: '4m',
      vus: 200,
      duration: '2m',
    },
  }
}

const body = JSON.stringify({
  to: "554499778888",
  text: "hello world load test"
});

const httpOptions = {
  headers: {
    "Content-Type": "application/json",
    Authorization: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhcHBfbmFtZSI6Im15LWFwcCIsImlhdCI6MTU5ODEzNzQ1MCwiZXhwIjoxNjA4NTA1NDUwfQ.iXxubUQad4_556nd4haqORl4ZxE0z5gH2RP00n6ISx8"
  }
};

export default function () {
  http.post('http://0.0.0.0:3000', body, httpOptions);
}