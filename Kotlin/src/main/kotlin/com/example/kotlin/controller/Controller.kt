package com.example.kotlin.controller

import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Primary
import org.springframework.core.io.ClassPathResource
import org.springframework.http.MediaType
import org.springframework.http.ResponseEntity
import org.springframework.http.client.MultipartBodyBuilder
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RestController
import org.springframework.web.reactive.function.BodyInserters
import org.springframework.web.reactive.function.client.WebClient
import org.springframework.web.reactive.function.client.awaitBody

@RestController
class Controller(private val httpClient: WebClient) {
    @PostMapping(produces = [MediaType.IMAGE_PNG_VALUE])
    suspend fun guilds(): ResponseEntity<ByteArray> {
        val builder = MultipartBodyBuilder()
        builder.part("file", ClassPathResource("cat.png")).filename("test").contentType(MediaType.IMAGE_PNG)
        builder.part("json_payload", "{ \"Some Json Name\": \"Some Value\" }").contentType(MediaType.TEXT_PLAIN)

        val res = httpClient
            .post()
            .uri("localhost:9015")
            .accept(MediaType.IMAGE_PNG)
            .contentType(MediaType.MULTIPART_FORM_DATA)
            .body(BodyInserters.fromMultipartData(builder.build()))
            .retrieve()
            .awaitBody<ByteArray>()

        return ResponseEntity.ok(res)
    }
}