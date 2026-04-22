#[doc = "Register `SPIF10C` reader"]
pub type R = crate::R<Spif10cSpec>;
#[doc = "Register `SPIF10C` writer"]
pub type W = crate::W<Spif10cSpec>;
#[doc = "Field `ADDRCTL03` reader - ADDR_CTL03"]
pub type Addrctl03R = crate::FieldReader;
#[doc = "Field `ADDRCTL03` writer - ADDR_CTL03"]
pub type Addrctl03W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ADDR_CTL03"]
    #[inline(always)]
    pub fn addrctl03(&self) -> Addrctl03R {
        Addrctl03R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR_CTL03"]
    #[inline(always)]
    pub fn addrctl03(&mut self) -> Addrctl03W<Spif10cSpec> {
        Addrctl03W::new(self, 0)
    }
}
#[doc = "SPIF\\_ADDRCTL03\n\nYou can [`read`](crate::Reg::read) this register and get [`spif10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif10cSpec;
impl crate::RegisterSpec for Spif10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif10c::R`](R) reader structure"]
impl crate::Readable for Spif10cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif10c::W`](W) writer structure"]
impl crate::Writable for Spif10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF10C to value 0"]
impl crate::Resettable for Spif10cSpec {}
