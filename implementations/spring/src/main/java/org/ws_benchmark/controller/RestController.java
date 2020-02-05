package org.ws_benchmark.controller;

import org.ws_benchmark.model.RestResult;
import org.ws_benchmark.service.RestService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@Controller
@RequestMapping("")
public class RestController {
    @Autowired
    RestService restService;

    @GetMapping("/color")
    public ResponseEntity<List<RestResult>> getAll() {
        List<RestResult> listRest = restService.getAll();
        return new ResponseEntity<List<RestResult>>(listRest, HttpStatus.OK);
    }

}