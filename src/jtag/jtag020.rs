#[doc = "Register `JTAG020` reader"]
pub type R = crate::R<Jtag020Spec>;
#[doc = "Register `JTAG020` writer"]
pub type W = crate::W<Jtag020Spec>;
#[doc = "Field `ShiftData3100` reader - Shift Data \\[31:00\\]"]
pub type ShiftData3100R = crate::FieldReader<u32>;
#[doc = "Field `ShiftData3100` writer - Shift Data \\[31:00\\]"]
pub type ShiftData3100W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shift Data \\[31:00\\]"]
    #[inline(always)]
    pub fn shift_data3100(&self) -> ShiftData3100R {
        ShiftData3100R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Data \\[31:00\\]"]
    #[inline(always)]
    pub fn shift_data3100(&mut self) -> ShiftData3100W<Jtag020Spec> {
        ShiftData3100W::new(self, 0)
    }
}
#[doc = "Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag020Spec;
impl crate::RegisterSpec for Jtag020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag020::R`](R) reader structure"]
impl crate::Readable for Jtag020Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag020::W`](W) writer structure"]
impl crate::Writable for Jtag020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG020 to value 0"]
impl crate::Resettable for Jtag020Spec {}
