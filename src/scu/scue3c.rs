#[doc = "Register `SCUE3C` reader"]
pub type R = crate::R<Scue3cSpec>;
#[doc = "Register `SCUE3C` writer"]
pub type W = crate::W<Scue3cSpec>;
#[doc = "Field `SCUREGLOCK780` reader - SCU_REG_LOCK_780"]
pub type Scureglock780R = crate::BitReader;
#[doc = "Field `SCUREGLOCK780` writer - SCU_REG_LOCK_780"]
pub type Scureglock780W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK784` reader - SCU_REG_LOCK_784"]
pub type Scureglock784R = crate::BitReader;
#[doc = "Field `SCUREGLOCK784` writer - SCU_REG_LOCK_784"]
pub type Scureglock784W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK788` reader - SCU_REG_LOCK_788"]
pub type Scureglock788R = crate::BitReader;
#[doc = "Field `SCUREGLOCK788` writer - SCU_REG_LOCK_788"]
pub type Scureglock788W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK78C` reader - SCU_REG_LOCK_78C"]
pub type Scureglock78cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK78C` writer - SCU_REG_LOCK_78C"]
pub type Scureglock78cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK790` reader - SCU_REG_LOCK_790"]
pub type Scureglock790R = crate::BitReader;
#[doc = "Field `SCUREGLOCK790` writer - SCU_REG_LOCK_790"]
pub type Scureglock790W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK794` reader - SCU_REG_LOCK_794"]
pub type Scureglock794R = crate::BitReader;
#[doc = "Field `SCUREGLOCK794` writer - SCU_REG_LOCK_794"]
pub type Scureglock794W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK798` reader - SCU_REG_LOCK_798"]
pub type Scureglock798R = crate::BitReader;
#[doc = "Field `SCUREGLOCK798` writer - SCU_REG_LOCK_798"]
pub type Scureglock798W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SCUREGLOCK7C0` reader - SCU_REG_LOCK_7C0"]
pub type Scureglock7c0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK7C0` writer - SCU_REG_LOCK_7C0"]
pub type Scureglock7c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK7C4` reader - SCU_REG_LOCK_7C4"]
pub type Scureglock7c4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK7C4` writer - SCU_REG_LOCK_7C4"]
pub type Scureglock7c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK7C8` reader - SCU_REG_LOCK_7C8"]
pub type Scureglock7c8R = crate::BitReader;
#[doc = "Field `SCUREGLOCK7C8` writer - SCU_REG_LOCK_7C8"]
pub type Scureglock7c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK7CC` reader - SCU_REG_LOCK_7CC"]
pub type Scureglock7ccR = crate::BitReader;
#[doc = "Field `SCUREGLOCK7CC` writer - SCU_REG_LOCK_7CC"]
pub type Scureglock7ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK7D0` reader - SCU_REG_LOCK_7D0"]
pub type Scureglock7d0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK7D0` writer - SCU_REG_LOCK_7D0"]
pub type Scureglock7d0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK7D4` reader - SCU_REG_LOCK_7D4"]
pub type Scureglock7d4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK7D4` writer - SCU_REG_LOCK_7D4"]
pub type Scureglock7d4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK7D8` reader - SCU_REG_LOCK_7D8"]
pub type Scureglock7d8R = crate::BitReader;
#[doc = "Field `SCUREGLOCK7D8` writer - SCU_REG_LOCK_7D8"]
pub type Scureglock7d8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_780"]
    #[inline(always)]
    pub fn scureglock780(&self) -> Scureglock780R {
        Scureglock780R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_784"]
    #[inline(always)]
    pub fn scureglock784(&self) -> Scureglock784R {
        Scureglock784R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_788"]
    #[inline(always)]
    pub fn scureglock788(&self) -> Scureglock788R {
        Scureglock788R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_78C"]
    #[inline(always)]
    pub fn scureglock78c(&self) -> Scureglock78cR {
        Scureglock78cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_790"]
    #[inline(always)]
    pub fn scureglock790(&self) -> Scureglock790R {
        Scureglock790R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_794"]
    #[inline(always)]
    pub fn scureglock794(&self) -> Scureglock794R {
        Scureglock794R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_798"]
    #[inline(always)]
    pub fn scureglock798(&self) -> Scureglock798R {
        Scureglock798R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_7C0"]
    #[inline(always)]
    pub fn scureglock7c0(&self) -> Scureglock7c0R {
        Scureglock7c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_7C4"]
    #[inline(always)]
    pub fn scureglock7c4(&self) -> Scureglock7c4R {
        Scureglock7c4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_LOCK_7C8"]
    #[inline(always)]
    pub fn scureglock7c8(&self) -> Scureglock7c8R {
        Scureglock7c8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_LOCK_7CC"]
    #[inline(always)]
    pub fn scureglock7cc(&self) -> Scureglock7ccR {
        Scureglock7ccR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_7D0"]
    #[inline(always)]
    pub fn scureglock7d0(&self) -> Scureglock7d0R {
        Scureglock7d0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_7D4"]
    #[inline(always)]
    pub fn scureglock7d4(&self) -> Scureglock7d4R {
        Scureglock7d4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_LOCK_7D8"]
    #[inline(always)]
    pub fn scureglock7d8(&self) -> Scureglock7d8R {
        Scureglock7d8R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_780"]
    #[inline(always)]
    pub fn scureglock780(&mut self) -> Scureglock780W<Scue3cSpec> {
        Scureglock780W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_784"]
    #[inline(always)]
    pub fn scureglock784(&mut self) -> Scureglock784W<Scue3cSpec> {
        Scureglock784W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_788"]
    #[inline(always)]
    pub fn scureglock788(&mut self) -> Scureglock788W<Scue3cSpec> {
        Scureglock788W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_78C"]
    #[inline(always)]
    pub fn scureglock78c(&mut self) -> Scureglock78cW<Scue3cSpec> {
        Scureglock78cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_790"]
    #[inline(always)]
    pub fn scureglock790(&mut self) -> Scureglock790W<Scue3cSpec> {
        Scureglock790W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_794"]
    #[inline(always)]
    pub fn scureglock794(&mut self) -> Scureglock794W<Scue3cSpec> {
        Scureglock794W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_798"]
    #[inline(always)]
    pub fn scureglock798(&mut self) -> Scureglock798W<Scue3cSpec> {
        Scureglock798W::new(self, 6)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_7C0"]
    #[inline(always)]
    pub fn scureglock7c0(&mut self) -> Scureglock7c0W<Scue3cSpec> {
        Scureglock7c0W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_7C4"]
    #[inline(always)]
    pub fn scureglock7c4(&mut self) -> Scureglock7c4W<Scue3cSpec> {
        Scureglock7c4W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_LOCK_7C8"]
    #[inline(always)]
    pub fn scureglock7c8(&mut self) -> Scureglock7c8W<Scue3cSpec> {
        Scureglock7c8W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_LOCK_7CC"]
    #[inline(always)]
    pub fn scureglock7cc(&mut self) -> Scureglock7ccW<Scue3cSpec> {
        Scureglock7ccW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_7D0"]
    #[inline(always)]
    pub fn scureglock7d0(&mut self) -> Scureglock7d0W<Scue3cSpec> {
        Scureglock7d0W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_7D4"]
    #[inline(always)]
    pub fn scureglock7d4(&mut self) -> Scureglock7d4W<Scue3cSpec> {
        Scureglock7d4W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_LOCK_7D8"]
    #[inline(always)]
    pub fn scureglock7d8(&mut self) -> Scureglock7d8W<Scue3cSpec> {
        Scureglock7d8W::new(self, 22)
    }
}
#[doc = "Write Protection 16 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue3cSpec;
impl crate::RegisterSpec for Scue3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue3c::R`](R) reader structure"]
impl crate::Readable for Scue3cSpec {}
#[doc = "`write(|w| ..)` method takes [`scue3c::W`](W) writer structure"]
impl crate::Writable for Scue3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE3C to value 0"]
impl crate::Resettable for Scue3cSpec {}
