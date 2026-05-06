#[doc = "Register `I2C_FILTER_THR07C` reader"]
pub type R = crate::R<I2cFilterThr07cSpec>;
#[doc = "Register `I2C_FILTER_THR07C` writer"]
pub type W = crate::W<I2cFilterThr07cSpec>;
#[doc = "Field `ELOGCNT` reader - ELOG_CNT"]
pub type ElogcntR = crate::FieldReader;
#[doc = "Field `ELOGCNT` writer - ELOG_CNT"]
pub type ElogcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ELOG_CNT"]
    #[inline(always)]
    pub fn elogcnt(&self) -> ElogcntR {
        ElogcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ELOG_CNT"]
    #[inline(always)]
    pub fn elogcnt(&mut self) -> ElogcntW<I2cFilterThr07cSpec> {
        ElogcntW::new(self, 0)
    }
}
#[doc = "I2CF\\_ELOG\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_filter_thr07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_filter_thr07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cFilterThr07cSpec;
impl crate::RegisterSpec for I2cFilterThr07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_thr07c::R`](R) reader structure"]
impl crate::Readable for I2cFilterThr07cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_thr07c::W`](W) writer structure"]
impl crate::Writable for I2cFilterThr07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_FILTER_THR07C to value 0"]
impl crate::Resettable for I2cFilterThr07cSpec {}
