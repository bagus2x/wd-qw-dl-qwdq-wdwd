import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
    vus: 2000, // Jumlah virtual users
    duration: '30s', // Durasi tes
};

export default function () {
    let url = 'http://localhost:8080/api/v1/user';
    let params = {
        headers: {
            'Authorization': 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImJhZ3VzQGdtYWlsLmNvbSIsImV4cCI6MTc0MDM3NDYzOSwiaWF0IjoxNzQwMzc0MDM5LCJzdWIiOiIwMTk1MzA3Ni01YmIwLTdjZjItOTIxOS0wYTRjMTAyZTVkYWIifQ.Q64aoYJq9VDC1vO1vsBKYo1HNiFA6iRoeyUDvhQdPzA',
        },
    };

    let res = http.get(url, params);

    check(res, {
        'status is 200': (r) => r.status === 200,
        'response is not empty': (r) => r.body.length > 0,
    });
}