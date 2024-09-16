/**
 ******************************************************************************
 * @file           : main.h
 * @brief          : Header for main.c file.
 *                   This file contains the common defines of the application.
 ******************************************************************************
 *
 * Copyright (c) 2023 Rizal Achmad Pahlevi <echo 'cml6YWwuYWhtYWRwQGdtYWlsLmNvbQo=' | base64 -d>
 * All rights reserved.
 *
 * This software is licensed under terms that can be found in the LICENSE file
 * in the root directory of this software component.
 * If no LICENSE file comes with this software, it is provided AS-IS.
 *
 ******************************************************************************
 */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __DYNO_H
#define __DYNO_H
#include <stdint.h>

#ifndef MAX_PPR_ENCODER
#define MAX_PPR_ENCODER 360 // default max pulse
#endif
#ifndef MAX_RPM_ENCODER
#define MAX_RPM_ENCODER 6000
#endif
#ifndef PERIOD_SEND_DATA_MS
#define PERIOD_SEND_DATA_MS 200
#endif

typedef struct {
    uint32_t period;
    uint32_t pulse_enc_max;
    uint32_t pulse_enc;
    uint32_t pulse_enc_z;
    uint32_t pulse_rpm;
    float temperature;
} DataDyno;

#define DYNO_SIZE_DATA sizeof(DataDyno)

void datadyno_reset(DataDyno *d);
void datadyno_send(DataDyno *d);
int MAX6675_Temp(float *data_temp);

#endif

#ifdef DYNO_IMPLEMENTATION
#define BUFFER_DATA_SIZE DYNO_SIZE_DATA + 1
static uint8_t BUFFER_DATA[BUFFER_DATA_SIZE] = {[DYNO_SIZE_DATA] = '\n'};
static DataDyno dyno = {0};

void datadyno_send(DataDyno *d) {
    memcpy(&BUFFER_DATA[0], d, DYNO_SIZE_DATA);
    CDC_Transmit_FS(&BUFFER_DATA[0], BUFFER_DATA_SIZE);
}

void datadyno_reset(DataDyno *d) {
    d->period = 0;
    d->pulse_enc_max = MAX_PPR_ENCODER;
    d->pulse_enc = 0;
    d->pulse_enc_z = 0;
    d->pulse_rpm = 0;
    d->temperature = 0.0;
}

int MAX6675_Temp(float *data_temp) {
    static uint8_t data_RX[2] = {0};
    HAL_StatusTypeDef ret = HAL_OK;

    // Waits for Chip Ready(according to Datasheet, the max time for conversion is 220ms)
    RESET_PIN(MAX6675_CS_GPIO_Port, MAX6675_CS_Pin);
    ret = HAL_SPI_Receive(&hspi_max6675, data_RX, 1, 250);
    SET_PIN(MAX6675_CS_GPIO_Port, MAX6675_CS_Pin);
    if (ret != HAL_OK)
        return ret;

    // convert [uint8_t; 2] to  uint16_t value
    uint16_t raw = (uint16_t)(((uint16_t)data_RX[0] << 8) | data_RX[1]);
    // State of Connecting
    if (((raw >> 2) & 0x01) == 0) {
        *data_temp = ((float)(raw >> 3)) * 0.25;
        return HAL_OK;
    }
    return ret;
}
#endif
