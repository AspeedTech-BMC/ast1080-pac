#[doc = "Register `I2C70` reader"]
pub type R = crate::R<I2c70Spec>;
#[doc = "Register `I2C70` writer"]
pub type W = crate::W<I2c70Spec>;
#[doc = "Field `TACST` reader - TACST"]
pub type TacstR = crate::FieldReader;
#[doc = "Field `TACST` writer - TACST"]
pub type TacstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THDSTA` reader - THDSTA"]
pub type ThdstaR = crate::FieldReader;
#[doc = "Field `THDSTA` writer - THDSTA"]
pub type ThdstaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TBUF` reader - TBUF"]
pub type TbufR = crate::FieldReader;
#[doc = "Field `TBUF` writer - TBUF"]
pub type TbufW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSUDAT` reader - TSUDAT"]
pub type TsudatR = crate::FieldReader;
#[doc = "Field `TSUDAT` writer - TSUDAT"]
pub type TsudatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IOCFG` reader - IO_CFG"]
pub type IocfgR = crate::FieldReader;
#[doc = "Field `IOCFG` writer - IO_CFG"]
pub type IocfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TACST"]
    #[inline(always)]
    pub fn tacst(&self) -> TacstR {
        TacstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - THDSTA"]
    #[inline(always)]
    pub fn thdsta(&self) -> ThdstaR {
        ThdstaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TBUF"]
    #[inline(always)]
    pub fn tbuf(&self) -> TbufR {
        TbufR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TSUDAT"]
    #[inline(always)]
    pub fn tsudat(&self) -> TsudatR {
        TsudatR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - IO_CFG"]
    #[inline(always)]
    pub fn iocfg(&self) -> IocfgR {
        IocfgR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TACST"]
    #[inline(always)]
    pub fn tacst(&mut self) -> TacstW<I2c70Spec> {
        TacstW::new(self, 0)
    }
    #[doc = "Bits 4:7 - THDSTA"]
    #[inline(always)]
    pub fn thdsta(&mut self) -> ThdstaW<I2c70Spec> {
        ThdstaW::new(self, 4)
    }
    #[doc = "Bits 8:11 - TBUF"]
    #[inline(always)]
    pub fn tbuf(&mut self) -> TbufW<I2c70Spec> {
        TbufW::new(self, 8)
    }
    #[doc = "Bits 12:15 - TSUDAT"]
    #[inline(always)]
    pub fn tsudat(&mut self) -> TsudatW<I2c70Spec> {
        TsudatW::new(self, 12)
    }
    #[doc = "Bits 16:19 - IO_CFG"]
    #[inline(always)]
    pub fn iocfg(&mut self) -> IocfgW<I2c70Spec> {
        IocfgW::new(self, 16)
    }
}
#[doc = "MISC configuration for AC timing0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c70::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c70::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c70Spec;
impl crate::RegisterSpec for I2c70Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c70::R`](R) reader structure"]
impl crate::Readable for I2c70Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c70::W`](W) writer structure"]
impl crate::Writable for I2c70Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C70 to value 0x2fff"]
impl crate::Resettable for I2c70Spec {
    const RESET_VALUE: u32 = 0x2fff;
}
