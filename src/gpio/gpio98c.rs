#[doc = "Register `GPIO98C` reader"]
pub type R = crate::R<Gpio98cSpec>;
#[doc = "Register `GPIO98C` writer"]
pub type W = crate::W<Gpio98cSpec>;
#[doc = "Field `GPIO124ReadPrivilegeOfMaster` reader - GPIO124 Read Privilege of Master"]
pub type Gpio124readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO124ReadPrivilegeOfMaster` writer - GPIO124 Read Privilege of Master"]
pub type Gpio124readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO125ReadPrivilegeOfMaster` reader - GPIO125 Read Privilege of Master"]
pub type Gpio125readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO125ReadPrivilegeOfMaster` writer - GPIO125 Read Privilege of Master"]
pub type Gpio125readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO126ReadPrivilegeOfMaster` reader - GPIO126 Read Privilege of Master"]
pub type Gpio126readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO126ReadPrivilegeOfMaster` writer - GPIO126 Read Privilege of Master"]
pub type Gpio126readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO127ReadPrivilegeOfMaster` reader - GPIO127 Read Privilege of Master"]
pub type Gpio127readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO127ReadPrivilegeOfMaster` writer - GPIO127 Read Privilege of Master"]
pub type Gpio127readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO124 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio124read_privilege_of_master(&self) -> Gpio124readPrivilegeOfMasterR {
        Gpio124readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO125 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio125read_privilege_of_master(&self) -> Gpio125readPrivilegeOfMasterR {
        Gpio125readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO126 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio126read_privilege_of_master(&self) -> Gpio126readPrivilegeOfMasterR {
        Gpio126readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO127 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio127read_privilege_of_master(&self) -> Gpio127readPrivilegeOfMasterR {
        Gpio127readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO124 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio124read_privilege_of_master(
        &mut self,
    ) -> Gpio124readPrivilegeOfMasterW<Gpio98cSpec> {
        Gpio124readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO125 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio125read_privilege_of_master(
        &mut self,
    ) -> Gpio125readPrivilegeOfMasterW<Gpio98cSpec> {
        Gpio125readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO126 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio126read_privilege_of_master(
        &mut self,
    ) -> Gpio126readPrivilegeOfMasterW<Gpio98cSpec> {
        Gpio126readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO127 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio127read_privilege_of_master(
        &mut self,
    ) -> Gpio127readPrivilegeOfMasterW<Gpio98cSpec> {
        Gpio127readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#31\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio98c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio98c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio98cSpec;
impl crate::RegisterSpec for Gpio98cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio98c::R`](R) reader structure"]
impl crate::Readable for Gpio98cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio98c::W`](W) writer structure"]
impl crate::Writable for Gpio98cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO98C to value 0xffff_ffff"]
impl crate::Resettable for Gpio98cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
