#[doc = "Register `JTAG000` reader"]
pub type R = crate::R<Jtag000Spec>;
#[doc = "Register `JTAG000` writer"]
pub type W = crate::W<Jtag000Spec>;
#[doc = "Field `DataPortReg` reader - Data Port register"]
pub type DataPortRegR = crate::FieldReader<u32>;
#[doc = "Field `DataPortReg` writer - Data Port register"]
pub type DataPortRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Port register"]
    #[inline(always)]
    pub fn data_port_reg(&self) -> DataPortRegR {
        DataPortRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Port register"]
    #[inline(always)]
    pub fn data_port_reg(&mut self) -> DataPortRegW<Jtag000Spec> {
        DataPortRegW::new(self, 0)
    }
}
#[doc = "Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag000Spec;
impl crate::RegisterSpec for Jtag000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag000::R`](R) reader structure"]
impl crate::Readable for Jtag000Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag000::W`](W) writer structure"]
impl crate::Writable for Jtag000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG000 to value 0"]
impl crate::Resettable for Jtag000Spec {}
