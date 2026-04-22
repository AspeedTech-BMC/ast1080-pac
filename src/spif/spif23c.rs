#[doc = "Register `SPIF23C` reader"]
pub type R = crate::R<Spif23cSpec>;
#[doc = "Register `SPIF23C` writer"]
pub type W = crate::W<Spif23cSpec>;
#[doc = "Field `ADDRLBND15` reader - ADDR_LBND15"]
pub type Addrlbnd15R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND15` writer - ADDR_LBND15"]
pub type Addrlbnd15W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND15` reader - ADDR_UBND15"]
pub type Addrubnd15R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND15` writer - ADDR_UBND15"]
pub type Addrubnd15W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND15"]
    #[inline(always)]
    pub fn addrlbnd15(&self) -> Addrlbnd15R {
        Addrlbnd15R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND15"]
    #[inline(always)]
    pub fn addrubnd15(&self) -> Addrubnd15R {
        Addrubnd15R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND15"]
    #[inline(always)]
    pub fn addrlbnd15(&mut self) -> Addrlbnd15W<Spif23cSpec> {
        Addrlbnd15W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND15"]
    #[inline(always)]
    pub fn addrubnd15(&mut self) -> Addrubnd15W<Spif23cSpec> {
        Addrubnd15W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND15\n\nYou can [`read`](crate::Reg::read) this register and get [`spif23c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif23c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif23cSpec;
impl crate::RegisterSpec for Spif23cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif23c::R`](R) reader structure"]
impl crate::Readable for Spif23cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif23c::W`](W) writer structure"]
impl crate::Writable for Spif23cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF23C to value 0"]
impl crate::Resettable for Spif23cSpec {}
