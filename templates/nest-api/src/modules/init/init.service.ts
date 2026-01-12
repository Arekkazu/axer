import { Injectable } from '@nestjs/common';
import { CreateInitDto } from './dto/create-init.dto.js';
import { UpdateInitDto } from './dto/update-init.dto';

@Injectable()
export class InitService {
  index() {
    return `Hello Start Dev :)`;
  }
}
