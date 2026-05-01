#[doc = "Register `GPIO820` reader"]
pub type R = crate::R<Gpio820Spec>;
#[doc = "Register `GPIO820` writer"]
pub type W = crate::W<Gpio820Spec>;
#[doc = "Field `GPIO016WrPrivilegeOfMaster` reader - GPIO016 Write Privilege of Master"]
pub type Gpio016wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO016WrPrivilegeOfMaster` writer - GPIO016 Write Privilege of Master"]
pub type Gpio016wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO017WrPrivilegeOfMaster` reader - GPIO017 Write Privilege of Master"]
pub type Gpio017wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO017WrPrivilegeOfMaster` writer - GPIO017 Write Privilege of Master"]
pub type Gpio017wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO018WrPrivilegeOfMaster` reader - GPIO018 Write Privilege of Master"]
pub type Gpio018wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO018WrPrivilegeOfMaster` writer - GPIO018 Write Privilege of Master"]
pub type Gpio018wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO019WrPrivilegeOfMaster` reader - GPIO019 Write Privilege of Master"]
pub type Gpio019wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO019WrPrivilegeOfMaster` writer - GPIO019 Write Privilege of Master"]
pub type Gpio019wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO016 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio016wr_privilege_of_master(&self) -> Gpio016wrPrivilegeOfMasterR {
        Gpio016wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO017 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio017wr_privilege_of_master(&self) -> Gpio017wrPrivilegeOfMasterR {
        Gpio017wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO018 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio018wr_privilege_of_master(&self) -> Gpio018wrPrivilegeOfMasterR {
        Gpio018wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO019 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio019wr_privilege_of_master(&self) -> Gpio019wrPrivilegeOfMasterR {
        Gpio019wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO016 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio016wr_privilege_of_master(&mut self) -> Gpio016wrPrivilegeOfMasterW<Gpio820Spec> {
        Gpio016wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO017 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio017wr_privilege_of_master(&mut self) -> Gpio017wrPrivilegeOfMasterW<Gpio820Spec> {
        Gpio017wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO018 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio018wr_privilege_of_master(&mut self) -> Gpio018wrPrivilegeOfMasterW<Gpio820Spec> {
        Gpio018wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO019 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio019wr_privilege_of_master(&mut self) -> Gpio019wrPrivilegeOfMasterW<Gpio820Spec> {
        Gpio019wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio820::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio820::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio820Spec;
impl crate::RegisterSpec for Gpio820Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio820::R`](R) reader structure"]
impl crate::Readable for Gpio820Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio820::W`](W) writer structure"]
impl crate::Writable for Gpio820Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO820 to value 0xffff_ffff"]
impl crate::Resettable for Gpio820Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
