package org.ws_benchmark.service;

import org.ws_benchmark.model.Rest;
import org.ws_benchmark.model.RestResult;

import java.util.List;

public interface RestService {
    List<RestResult> getAll();
}
