#[doc = "Register `JTAG004` reader"]
pub type R = crate::R<Jtag004Spec>;
#[doc = "Register `JTAG004` writer"]
pub type W = crate::W<Jtag004Spec>;
#[doc = "Field `DataPortReg1` reader - Data Port register"]
pub type DataPortReg1R = crate::FieldReader<u32>;
#[doc = "Field `DataPortReg1` writer - Data Port register"]
pub type DataPortReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Port register"]
    #[inline(always)]
    pub fn data_port_reg1(&self) -> DataPortReg1R {
        DataPortReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Port register"]
    #[inline(always)]
    pub fn data_port_reg1(&mut self) -> DataPortReg1W<Jtag004Spec> {
        DataPortReg1W::new(self, 0)
    }
}
#[doc = "Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag004Spec;
impl crate::RegisterSpec for Jtag004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag004::R`](R) reader structure"]
impl crate::Readable for Jtag004Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag004::W`](W) writer structure"]
impl crate::Writable for Jtag004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG004 to value 0"]
impl crate::Resettable for Jtag004Spec {}
