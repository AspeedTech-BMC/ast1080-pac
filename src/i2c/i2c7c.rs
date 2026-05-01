#[doc = "Register `I2C7C` reader"]
pub type R = crate::R<I2c7cSpec>;
#[doc = "Register `I2C7C` writer"]
pub type W = crate::W<I2c7cSpec>;
#[doc = "Field `HSACTIME` reader - HS_ACTIME"]
pub type HsactimeR = crate::FieldReader<u32>;
#[doc = "Field `HSACTIME` writer - HS_ACTIME"]
pub type HsactimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HS_ACTIME"]
    #[inline(always)]
    pub fn hsactime(&self) -> HsactimeR {
        HsactimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HS_ACTIME"]
    #[inline(always)]
    pub fn hsactime(&mut self) -> HsactimeW<I2c7cSpec> {
        HsactimeW::new(self, 0)
    }
}
#[doc = "I2CC\\_HS\\_ACTIME\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c7c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c7c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c7cSpec;
impl crate::RegisterSpec for I2c7cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c7c::R`](R) reader structure"]
impl crate::Readable for I2c7cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c7c::W`](W) writer structure"]
impl crate::Writable for I2c7cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C7C to value 0x00ef_f000"]
impl crate::Resettable for I2c7cSpec {
    const RESET_VALUE: u32 = 0x00ef_f000;
}
