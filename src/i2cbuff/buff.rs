#[doc = "Register `BUFF[%s]` reader"]
pub type R = crate::R<BuffSpec>;
#[doc = "Register `BUFF[%s]` writer"]
pub type W = crate::W<BuffSpec>;
#[doc = "Field `Data` reader - data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `Data` writer - data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<BuffSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "I2C Buffer Mode Buff\n\nYou can [`read`](crate::Reg::read) this register and get [`buff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuffSpec;
impl crate::RegisterSpec for BuffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buff::R`](R) reader structure"]
impl crate::Readable for BuffSpec {}
#[doc = "`write(|w| ..)` method takes [`buff::W`](W) writer structure"]
impl crate::Writable for BuffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFF[%s] to value 0"]
impl crate::Resettable for BuffSpec {}
