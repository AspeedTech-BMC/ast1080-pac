#[doc = "Register `SGPIO00C` reader"]
pub type R = crate::R<Sgpio00cSpec>;
#[doc = "Register `SGPIO00C` writer"]
pub type W = crate::W<Sgpio00cSpec>;
#[doc = "Field `InputValueOfSGPIO64` reader - Input value of SGPIO_64"]
pub type InputValueOfSgpio64R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO65` reader - Input value of SGPIO_65"]
pub type InputValueOfSgpio65R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO66` reader - Input value of SGPIO_66"]
pub type InputValueOfSgpio66R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO67` reader - Input value of SGPIO_67"]
pub type InputValueOfSgpio67R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO68` reader - Input value of SGPIO_68"]
pub type InputValueOfSgpio68R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO69` reader - Input value of SGPIO_69"]
pub type InputValueOfSgpio69R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO70` reader - Input value of SGPIO_70"]
pub type InputValueOfSgpio70R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO71` reader - Input value of SGPIO_71"]
pub type InputValueOfSgpio71R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO72` reader - Input value of SGPIO_72"]
pub type InputValueOfSgpio72R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO73` reader - Input value of SGPIO_73"]
pub type InputValueOfSgpio73R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO74` reader - Input value of SGPIO_74"]
pub type InputValueOfSgpio74R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO75` reader - Input value of SGPIO_75"]
pub type InputValueOfSgpio75R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO76` reader - Input value of SGPIO_76"]
pub type InputValueOfSgpio76R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO77` reader - Input value of SGPIO_77"]
pub type InputValueOfSgpio77R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO78` reader - Input value of SGPIO_78"]
pub type InputValueOfSgpio78R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO79` reader - Input value of SGPIO_79"]
pub type InputValueOfSgpio79R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO80` reader - Input value of SGPIO_80"]
pub type InputValueOfSgpio80R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO81` reader - Input value of SGPIO_81"]
pub type InputValueOfSgpio81R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO82` reader - Input value of SGPIO_82"]
pub type InputValueOfSgpio82R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO83` reader - Input value of SGPIO_83"]
pub type InputValueOfSgpio83R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO84` reader - Input value of SGPIO_84"]
pub type InputValueOfSgpio84R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO85` reader - Input value of SGPIO_85"]
pub type InputValueOfSgpio85R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO86` reader - Input value of SGPIO_86"]
pub type InputValueOfSgpio86R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO87` reader - Input value of SGPIO_87"]
pub type InputValueOfSgpio87R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO88` reader - Input value of SGPIO_88"]
pub type InputValueOfSgpio88R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO89` reader - Input value of SGPIO_89"]
pub type InputValueOfSgpio89R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO90` reader - Input value of SGPIO_90"]
pub type InputValueOfSgpio90R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO91` reader - Input value of SGPIO_91"]
pub type InputValueOfSgpio91R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO92` reader - Input value of SGPIO_92"]
pub type InputValueOfSgpio92R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO93` reader - Input value of SGPIO_93"]
pub type InputValueOfSgpio93R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO94` reader - Input value of SGPIO_94"]
pub type InputValueOfSgpio94R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO95` reader - Input value of SGPIO_95"]
pub type InputValueOfSgpio95R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input value of SGPIO_64"]
    #[inline(always)]
    pub fn input_value_of_sgpio64(&self) -> InputValueOfSgpio64R {
        InputValueOfSgpio64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input value of SGPIO_65"]
    #[inline(always)]
    pub fn input_value_of_sgpio65(&self) -> InputValueOfSgpio65R {
        InputValueOfSgpio65R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input value of SGPIO_66"]
    #[inline(always)]
    pub fn input_value_of_sgpio66(&self) -> InputValueOfSgpio66R {
        InputValueOfSgpio66R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input value of SGPIO_67"]
    #[inline(always)]
    pub fn input_value_of_sgpio67(&self) -> InputValueOfSgpio67R {
        InputValueOfSgpio67R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input value of SGPIO_68"]
    #[inline(always)]
    pub fn input_value_of_sgpio68(&self) -> InputValueOfSgpio68R {
        InputValueOfSgpio68R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input value of SGPIO_69"]
    #[inline(always)]
    pub fn input_value_of_sgpio69(&self) -> InputValueOfSgpio69R {
        InputValueOfSgpio69R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input value of SGPIO_70"]
    #[inline(always)]
    pub fn input_value_of_sgpio70(&self) -> InputValueOfSgpio70R {
        InputValueOfSgpio70R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input value of SGPIO_71"]
    #[inline(always)]
    pub fn input_value_of_sgpio71(&self) -> InputValueOfSgpio71R {
        InputValueOfSgpio71R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input value of SGPIO_72"]
    #[inline(always)]
    pub fn input_value_of_sgpio72(&self) -> InputValueOfSgpio72R {
        InputValueOfSgpio72R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input value of SGPIO_73"]
    #[inline(always)]
    pub fn input_value_of_sgpio73(&self) -> InputValueOfSgpio73R {
        InputValueOfSgpio73R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input value of SGPIO_74"]
    #[inline(always)]
    pub fn input_value_of_sgpio74(&self) -> InputValueOfSgpio74R {
        InputValueOfSgpio74R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input value of SGPIO_75"]
    #[inline(always)]
    pub fn input_value_of_sgpio75(&self) -> InputValueOfSgpio75R {
        InputValueOfSgpio75R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input value of SGPIO_76"]
    #[inline(always)]
    pub fn input_value_of_sgpio76(&self) -> InputValueOfSgpio76R {
        InputValueOfSgpio76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input value of SGPIO_77"]
    #[inline(always)]
    pub fn input_value_of_sgpio77(&self) -> InputValueOfSgpio77R {
        InputValueOfSgpio77R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input value of SGPIO_78"]
    #[inline(always)]
    pub fn input_value_of_sgpio78(&self) -> InputValueOfSgpio78R {
        InputValueOfSgpio78R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input value of SGPIO_79"]
    #[inline(always)]
    pub fn input_value_of_sgpio79(&self) -> InputValueOfSgpio79R {
        InputValueOfSgpio79R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input value of SGPIO_80"]
    #[inline(always)]
    pub fn input_value_of_sgpio80(&self) -> InputValueOfSgpio80R {
        InputValueOfSgpio80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input value of SGPIO_81"]
    #[inline(always)]
    pub fn input_value_of_sgpio81(&self) -> InputValueOfSgpio81R {
        InputValueOfSgpio81R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input value of SGPIO_82"]
    #[inline(always)]
    pub fn input_value_of_sgpio82(&self) -> InputValueOfSgpio82R {
        InputValueOfSgpio82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input value of SGPIO_83"]
    #[inline(always)]
    pub fn input_value_of_sgpio83(&self) -> InputValueOfSgpio83R {
        InputValueOfSgpio83R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input value of SGPIO_84"]
    #[inline(always)]
    pub fn input_value_of_sgpio84(&self) -> InputValueOfSgpio84R {
        InputValueOfSgpio84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input value of SGPIO_85"]
    #[inline(always)]
    pub fn input_value_of_sgpio85(&self) -> InputValueOfSgpio85R {
        InputValueOfSgpio85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input value of SGPIO_86"]
    #[inline(always)]
    pub fn input_value_of_sgpio86(&self) -> InputValueOfSgpio86R {
        InputValueOfSgpio86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input value of SGPIO_87"]
    #[inline(always)]
    pub fn input_value_of_sgpio87(&self) -> InputValueOfSgpio87R {
        InputValueOfSgpio87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input value of SGPIO_88"]
    #[inline(always)]
    pub fn input_value_of_sgpio88(&self) -> InputValueOfSgpio88R {
        InputValueOfSgpio88R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input value of SGPIO_89"]
    #[inline(always)]
    pub fn input_value_of_sgpio89(&self) -> InputValueOfSgpio89R {
        InputValueOfSgpio89R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input value of SGPIO_90"]
    #[inline(always)]
    pub fn input_value_of_sgpio90(&self) -> InputValueOfSgpio90R {
        InputValueOfSgpio90R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input value of SGPIO_91"]
    #[inline(always)]
    pub fn input_value_of_sgpio91(&self) -> InputValueOfSgpio91R {
        InputValueOfSgpio91R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input value of SGPIO_92"]
    #[inline(always)]
    pub fn input_value_of_sgpio92(&self) -> InputValueOfSgpio92R {
        InputValueOfSgpio92R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input value of SGPIO_93"]
    #[inline(always)]
    pub fn input_value_of_sgpio93(&self) -> InputValueOfSgpio93R {
        InputValueOfSgpio93R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input value of SGPIO_94"]
    #[inline(always)]
    pub fn input_value_of_sgpio94(&self) -> InputValueOfSgpio94R {
        InputValueOfSgpio94R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input value of SGPIO_95"]
    #[inline(always)]
    pub fn input_value_of_sgpio95(&self) -> InputValueOfSgpio95R {
        InputValueOfSgpio95R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial In Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio00cSpec;
impl crate::RegisterSpec for Sgpio00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio00c::R`](R) reader structure"]
impl crate::Readable for Sgpio00cSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio00c::W`](W) writer structure"]
impl crate::Writable for Sgpio00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO00C to value 0"]
impl crate::Resettable for Sgpio00cSpec {}
