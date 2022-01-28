package com.example.kotlin

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.context.annotation.Bean
import org.springframework.web.reactive.function.client.WebClient

@SpringBootApplication
class KotlinApplication

fun main(args: Array<String>) {
    runApplication<KotlinApplication>(*args)
}