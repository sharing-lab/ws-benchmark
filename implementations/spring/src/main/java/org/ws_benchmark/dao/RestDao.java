package org.ws_benchmark.dao;

import org.ws_benchmark.model.Rest;
import org.ws_benchmark.model.RestResult;

import java.util.List;

public interface RestDao {
    List<RestResult> getAll();
}