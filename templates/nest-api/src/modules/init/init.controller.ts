import {
  Controller,
  Get,
  Post,
  Body,
  Patch,
  Param,
  Delete,
} from '@nestjs/common';
import { InitService } from './init.service';
import { CreateInitDto } from './dto/create-init.dto';
import { UpdateInitDto } from './dto/update-init.dto';

@Controller('init')
export class InitController {
  constructor(private readonly initService: InitService) {}

  @Get()
  index() {
    return this.initService.index();
  }
}
