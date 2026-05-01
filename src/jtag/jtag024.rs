#[doc = "Register `JTAG024` reader"]
pub type R = crate::R<Jtag024Spec>;
#[doc = "Register `JTAG024` writer"]
pub type W = crate::W<Jtag024Spec>;
#[doc = "Field `ShiftData31001` reader - Shift Data \\[31:00\\]"]
pub type ShiftData31001R = crate::FieldReader<u32>;
#[doc = "Field `ShiftData31001` writer - Shift Data \\[31:00\\]"]
pub type ShiftData31001W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shift Data \\[31:00\\]"]
    #[inline(always)]
    pub fn shift_data31001(&self) -> ShiftData31001R {
        ShiftData31001R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Data \\[31:00\\]"]
    #[inline(always)]
    pub fn shift_data31001(&mut self) -> ShiftData31001W<Jtag024Spec> {
        ShiftData31001W::new(self, 0)
    }
}
#[doc = "Data Port register\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag024Spec;
impl crate::RegisterSpec for Jtag024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag024::R`](R) reader structure"]
impl crate::Readable for Jtag024Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag024::W`](W) writer structure"]
impl crate::Writable for Jtag024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG024 to value 0"]
impl crate::Resettable for Jtag024Spec {}
